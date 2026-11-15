# OCR offline da captura do cliente 7662 e comparação com um registro de `UI/strdef.bin`.
#
# Por que existe: a evidência do texto desenhado na tela era lida à mão no recorte. Aqui a
# leitura vira automática e reprodutível: `Windows.Media.Ocr` (offline, motor do Windows)
# transcreve a captura e o texto é comparado com o literal do registro extraído do binário do
# cliente por `tools/client/strdef.mjs --index N --json`. A comparação normaliza acento, caixa
# e pontuação (o motor de OCR erra `ã`/`ç` e o cliente desenha bitmap pequeno), e exige um
# mínimo de semelhança — abaixo disso o script sai com código 1.
#
# Uso:
#   powershell -NoProfile -ExecutionPolicy Bypass -File tools/client/ocr-texto.ps1 `
#     -Imagem target/login-recusa-1.jpg -Strdef 12
#   ... -Recorte 0,240,806,80     # x,y,largura,altura na imagem de origem
#   ... -Escala 4 -Minimo 0.85 -Json target/login-ocr.json
#
# Saída: JSON de uma linha em stdout (e no arquivo de `-Json`), com `lido`, `esperado`,
# `normalizado_*`, `semelhanca` e `ok`. Código de saída: 0 = confere, 1 = não confere,
# 2 = OCR indisponível nesta máquina (nada é afirmado).
param(
  [Parameter(Mandatory = $true)][string]$Imagem,
  [int]$Strdef = -1,
  # Texto esperado **fora** do `strdef.bin`: é o caso do painel `0x0101`, cujo literal viaja no
  # frame do servidor (ex.: "PIN incorreto. Tente novamente.") e não existe no binário do cliente.
  [string]$Texto = "",
  [double]$Minimo = 0.85,
  # Medido nos frames reais do 7662: 3x dá 0.857/0.93 e 4x dá 0.963 na mesma captura — o
  # bitmap pequeno do cliente precisa de 4x para o motor separar "senha" de "pergha".
  [int]$Escala = 4,
  # x,y,largura,altura na imagem de origem; vazio = imagem inteira.
  [int[]]$Recorte = @(),
  # Fatias horizontais sobrepostas quando não há `-Recorte`: o texto do cliente é uma faixa e o
  # motor junta faixas vizinhas quando recebe a janela inteira.
  [int]$Fatias = 6,
  [string]$Json = ""
)

$ErrorActionPreference = "Stop"
# O literal sai de `node` em UTF-8; sem isto o PowerShell 5.1 lê o stdout como cp1252 e
# "Conexão" chega como "Conex?o" — a comparação passaria a medir a codificação do console.
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$OutputEncoding = [System.Text.Encoding]::UTF8
$raiz = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$origem = if ([System.IO.Path]::IsPathRooted($Imagem)) { $Imagem } else { Join-Path $raiz $Imagem }

function Normaliza([string]$texto) {
  if ($null -eq $texto) { return "" }
  $decomposto = $texto.Normalize([System.Text.NormalizationForm]::FormD)
  $semAcento = -join ($decomposto.ToCharArray() | Where-Object {
      [System.Globalization.CharUnicodeInfo]::GetUnicodeCategory($_) -ne
      [System.Globalization.UnicodeCategory]::NonSpacingMark
    })
  # só letras/dígitos e espaço simples: o OCR troca vírgula por ponto e pontuação por espaço
  $limpo = ($semAcento.ToLowerInvariant() -replace "[^a-z0-9]+", " ") -replace "\s+", " "
  return $limpo.Trim()
}

function Distancia([string]$a, [string]$b) {
  $n = $a.Length; $m = $b.Length
  if ($n -eq 0) { return $m }
  if ($m -eq 0) { return $n }
  $anterior = New-Object int[] ($m + 1)
  $atual = New-Object int[] ($m + 1)
  for ($j = 0; $j -le $m; $j++) { $anterior[$j] = $j }
  for ($i = 1; $i -le $n; $i++) {
    $atual[0] = $i
    for ($j = 1; $j -le $m; $j++) {
      $custo = if ($a[$i - 1] -eq $b[$j - 1]) { 0 } else { 1 }
      $atual[$j] = [Math]::Min([Math]::Min($atual[$j - 1] + 1, $anterior[$j] + 1), $anterior[$j - 1] + $custo)
    }
    $troca = $anterior; $anterior = $atual; $atual = $troca
  }
  return $anterior[$m]
}

function Semelhanca([string]$esperado, [string]$lido) {
  if ($esperado.Length -eq 0) { return 0.0 }
  if ($lido.Contains($esperado)) { return 1.0 }
  # melhor janela de palavras do texto lido com tamanho próximo do esperado
  $alvo = $esperado.Split(" ").Count
  $palavras = $lido.Split(" ", [System.StringSplitOptions]::RemoveEmptyEntries)
  $melhor = 0.0
  for ($i = 0; $i -lt $palavras.Count; $i++) {
    for ($tam = [Math]::Max(1, $alvo - 3); $tam -le [Math]::Min($palavras.Count - $i, $alvo + 3); $tam++) {
      $janela = ($palavras[$i..($i + $tam - 1)] -join " ")
      $maior = [Math]::Max($esperado.Length, $janela.Length)
      $s = 1.0 - ([double](Distancia $esperado $janela) / $maior)
      if ($s -gt $melhor) { $melhor = $s }
    }
  }
  return [Math]::Round($melhor, 4)
}

function LeTextoOcr([string]$caminho) {
  Add-Type -AssemblyName System.Runtime.WindowsRuntime
  [void][Windows.Storage.StorageFile, Windows.Storage, ContentType = WindowsRuntime]
  [void][Windows.Graphics.Imaging.BitmapDecoder, Windows.Foundation, ContentType = WindowsRuntime]
  [void][Windows.Media.Ocr.OcrEngine, Windows.Foundation, ContentType = WindowsRuntime]
  [void][Windows.Globalization.Language, Windows.Foundation, ContentType = WindowsRuntime]

  $asTask = ([System.WindowsRuntimeSystemExtensions].GetMethods() | Where-Object {
      $_.Name -eq "AsTask" -and $_.GetParameters().Count -eq 1 -and
      $_.GetParameters()[0].ParameterType.Name -eq "IAsyncOperation``1"
    })[0]
  function Espera($operacao, $tipo) {
    $tarefa = $asTask.MakeGenericMethod($tipo).Invoke($null, @($operacao))
    $tarefa.Wait(-1) | Out-Null
    return $tarefa.Result
  }

  $arquivo = Espera ([Windows.Storage.StorageFile]::GetFileFromPathAsync($caminho)) ([Windows.Storage.StorageFile])
  $fluxo = Espera ($arquivo.OpenAsync([Windows.Storage.FileAccessMode]::Read)) ([Windows.Storage.Streams.IRandomAccessStream])
  $decodificador = Espera ([Windows.Graphics.Imaging.BitmapDecoder]::CreateAsync($fluxo)) ([Windows.Graphics.Imaging.BitmapDecoder])
  $bitmap = Espera ($decodificador.GetSoftwareBitmapAsync()) ([Windows.Graphics.Imaging.SoftwareBitmap])

  $motor = [Windows.Media.Ocr.OcrEngine]::TryCreateFromUserProfileLanguages()
  if ($null -eq $motor) {
    $motor = [Windows.Media.Ocr.OcrEngine]::TryCreateFromLanguage([Windows.Globalization.Language]::new("pt-BR"))
  }
  if ($null -eq $motor) { throw "OCR_UNAVAILABLE" }
  $resultado = Espera ($motor.RecognizeAsync($bitmap)) ([Windows.Media.Ocr.OcrResult])
  return (($resultado.Lines | ForEach-Object { $_.Text }) -join " ").Trim()
}

# recorte + ampliação com System.Drawing (o motor do Windows lê melhor 3-4x)
Add-Type -AssemblyName System.Drawing

# Regiões a ler: o `-Recorte` explícito, ou `-Fatias` faixas horizontais sobrepostas.
$imgInfo = [System.Drawing.Image]::FromFile($origem)
$larguraTotal = $imgInfo.Width; $alturaTotal = $imgInfo.Height
$imgInfo.Dispose()
$regioes = @()
if ($Recorte.Count -eq 4) {
  $regioes += , @($Recorte[0], $Recorte[1], $Recorte[2], $Recorte[3])
} elseif ($Fatias -gt 1) {
  $alturaFatia = [int][Math]::Ceiling($alturaTotal / $Fatias)
  $sobreposicao = [int]($alturaFatia / 2)
  for ($i = 0; $i -lt $Fatias; $i++) {
    $y = [Math]::Max(0, $i * $alturaFatia - $sobreposicao)
    $h = [Math]::Min($alturaFatia + 2 * $sobreposicao, $alturaTotal - $y)
    $regioes += , @(0, $y, $larguraTotal, $h)
  }
} else {
  $regioes += , @(0, 0, $larguraTotal, $alturaTotal)
}

$textos = @()
$indice = 0
foreach ($regiao in $regioes) {
  $tmp = Join-Path $raiz ("target/ocr-tmp-" + [System.IO.Path]::GetFileNameWithoutExtension($origem) + "-$indice.png")
  $img = [System.Drawing.Image]::FromFile($origem)
  try {
    $x = $regiao[0]; $y = $regiao[1]; $w = $regiao[2]; $h = $regiao[3]
    $x = [Math]::Max(0, [Math]::Min($x, $img.Width - 1))
    $y = [Math]::Max(0, [Math]::Min($y, $img.Height - 1))
    $w = [Math]::Min($w, $img.Width - $x); $h = [Math]::Min($h, $img.Height - $y)
    $bmp = New-Object System.Drawing.Bitmap([int]($w * $Escala), [int]($h * $Escala))
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
    $g.DrawImage($img, (New-Object System.Drawing.Rectangle(0, 0, $bmp.Width, $bmp.Height)),
      (New-Object System.Drawing.Rectangle($x, $y, $w, $h)), [System.Drawing.GraphicsUnit]::Pixel)
    $bmp.Save($tmp, [System.Drawing.Imaging.ImageFormat]::Png)
    $g.Dispose(); $bmp.Dispose()
  } finally { $img.Dispose() }

  try {
    $textos += , @($indice, (LeTextoOcr $tmp))
  } catch {
    if ("$($_.Exception.Message)".Contains("OCR_UNAVAILABLE")) {
      $saida = [ordered]@{ ok = $false; motivo = "ocr_indisponivel"; imagem = $Imagem; lido = "" }
      Remove-Item -Path $tmp -Force -ErrorAction SilentlyContinue
      break
    }
    throw
  }
  Remove-Item -Path $tmp -Force -ErrorAction SilentlyContinue
  $indice++
}

if (-not $saida) {
  $lido = (($textos | ForEach-Object { $_[1] }) -join " | ")
  if ($Texto -ne "") {
    $esperado = $Texto
    $normalEsperado = Normaliza $esperado
    $melhor = 0.0; $melhorIndice = -1; $melhorLido = ""
    foreach ($par in $textos) {
      $normalLido = Normaliza $par[1]
      $s = Semelhanca $normalEsperado $normalLido
      if ($s -gt $melhor) { $melhor = $s; $melhorIndice = $par[0]; $melhorLido = $par[1] }
    }
    $saida = [ordered]@{
      ok                   = ($melhor -ge $Minimo)
      imagem               = $Imagem
      regioes              = $regioes.Count
      fonte                = "texto"
      esperado             = $esperado
      lido                 = $melhorLido
      lido_todas_faixas    = $lido
      fatia                = $melhorIndice
      normalizado_esperado = $normalEsperado
      normalizado_lido     = (Normaliza $melhorLido)
      semelhanca           = $melhor
      minimo               = $Minimo
    }
  } elseif ($Strdef -ge 0) {
    $bruto = (& node (Join-Path $raiz "tools/client/strdef.mjs") --index $Strdef --json 2>$null | Out-String).Trim()
    if ($LASTEXITCODE -ne 0 -or -not $bruto) { throw "strdef --index $Strdef falhou" }
    $registro = $bruto | ConvertFrom-Json
    $esperado = $registro.texto
    $normalEsperado = Normaliza $esperado
    # melhor faixa: o texto pode estar em qualquer ponto da tela
    $melhor = 0.0; $melhorIndice = -1; $melhorLido = ""
    foreach ($par in $textos) {
      $normalLido = Normaliza $par[1]
      $s = Semelhanca $normalEsperado $normalLido
      if ($s -gt $melhor) { $melhor = $s; $melhorIndice = $par[0]; $melhorLido = $par[1] }
    }
    $saida = [ordered]@{
      ok                    = ($melhor -ge $Minimo)
      imagem                = $Imagem
      regioes               = $regioes.Count
      strdef_indice         = $Strdef
      strdef_offset         = ("0x{0:x}" -f [int]$registro.offset)
      esperado              = $esperado
      lido                  = $melhorLido
      lido_todas_faixas     = $lido
      fatia                 = $melhorIndice
      normalizado_esperado  = $normalEsperado
      normalizado_lido      = (Normaliza $melhorLido)
      semelhanca            = $melhor
      minimo                = $Minimo
    }
  } else {
    $saida = [ordered]@{ ok = $true; imagem = $Imagem; lido = $lido }
  }
}

$jsonTexto = $saida | ConvertTo-Json -Compress
if ($Json -ne "") {
  $destino = if ([System.IO.Path]::IsPathRooted($Json)) { $Json } else { Join-Path $raiz $Json }
  New-Item -ItemType Directory -Force -Path (Split-Path $destino) | Out-Null
  Set-Content -Path $destino -Value $jsonTexto -Encoding utf8
}
Write-Output $jsonTexto
if ($saida.motivo -eq "ocr_indisponivel") { exit 2 }
if ($saida.ok) { exit 0 } else { exit 1 }
