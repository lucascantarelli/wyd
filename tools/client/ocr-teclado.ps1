# OCR com **caixas** da captura do cliente 7662: localiza cada palavra desenhada na região e
# devolve o centro em coordenadas da janela, para o harness clicar.
#
# Por que existe: o diálogo "Insira Senha(2)" do 7662 é um teclado numérico com os dígitos
# **embaralhados** (medido em 2026-11-15: `1 3 9 / 6 4 / 7 5 2 0 8` na captura). Clicar por
# coordenada fixa digitaria o PIN errado — e cinco PINs errados bloqueiam a conta por 10 min
# (`accounts.pin_failures`/`pin_locked_until`). Aqui a posição de cada dígito sai do próprio
# frame, então o clique acompanha o embaralhamento.
#
# Uso:
#   powershell -NoProfile -ExecutionPolicy Bypass -File tools/client/ocr-teclado.ps1 `
#     -Imagem target/pinmapa-recusa-1.png -Regiao 250,250,300,190 -Json target/teclado.json
#
# Saída: JSON de uma linha com `palavras: [{texto, x, y, largura, altura}]` em coordenadas da
# imagem de origem (não da versão ampliada). Código 0 = leu, 2 = OCR indisponível.
param(
  [Parameter(Mandatory = $true)][string]$Imagem,
  # "x,y,largura,altura" na imagem de origem; vazio = imagem inteira. Texto (não `[int[]]`)
  # porque `powershell -File` entrega argumento como string e a conversão automática falha.
  [string]$Regiao = "",
  [int]$Escala = 4,
  [string]$Json = ""
)

$ErrorActionPreference = "Stop"
[Console]::OutputEncoding = [System.Text.Encoding]::UTF8
$raiz = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$origem = if ([System.IO.Path]::IsPathRooted($Imagem)) { $Imagem } else { Join-Path $raiz $Imagem }

Add-Type -AssemblyName System.Drawing

$recorte = @()
if ($Regiao.Trim() -ne "") {
  $recorte = @($Regiao.Split(",") | ForEach-Object { [int]$_.Trim() })
  if ($recorte.Count -ne 4) { throw "-Regiao pede x,y,largura,altura" }
}

$img = [System.Drawing.Image]::FromFile($origem)
try {
  if ($recorte.Count -eq 4) {
    $x = [Math]::Max(0, $recorte[0]); $y = [Math]::Max(0, $recorte[1])
    $w = [Math]::Min($recorte[2], $img.Width - $x); $h = [Math]::Min($recorte[3], $img.Height - $y)
  } else {
    $x = 0; $y = 0; $w = $img.Width; $h = $img.Height
  }
  $bmp = New-Object System.Drawing.Bitmap ([int]($w * $Escala)), ([int]($h * $Escala))
  $g = [System.Drawing.Graphics]::FromImage($bmp)
  $g.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
  $g.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality
  $g.DrawImage($img, (New-Object System.Drawing.Rectangle(0, 0, $bmp.Width, $bmp.Height)),
    (New-Object System.Drawing.Rectangle($x, $y, $w, $h)), [System.Drawing.GraphicsUnit]::Pixel)
  $tmp = Join-Path $raiz ("target/ocr-teclado-" + [System.IO.Path]::GetFileNameWithoutExtension($origem) + ".png")
  New-Item -ItemType Directory -Force -Path (Split-Path $tmp) | Out-Null
  $bmp.Save($tmp, [System.Drawing.Imaging.ImageFormat]::Png)
  $g.Dispose(); $bmp.Dispose()
} finally { $img.Dispose() }

function LePalavras([string]$caminho) {
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

  $palavras = @()
  foreach ($linha in $resultado.Lines) {
    foreach ($palavra in $linha.Words) {
      $r = $palavra.BoundingRect
      $palavras += , [ordered]@{
        texto   = $palavra.Text
        # de volta para a escala da imagem de origem, no sistema da imagem inteira
        x       = [Math]::Round($x + ($r.X + $r.Width / 2) / $Escala, 1)
        y       = [Math]::Round($y + ($r.Y + $r.Height / 2) / $Escala, 1)
        largura = [Math]::Round($r.Width / $Escala, 1)
        altura  = [Math]::Round($r.Height / $Escala, 1)
      }
    }
  }
  return $palavras
}

try {
  $palavras = LePalavras $tmp
} catch {
  if ("$($_.Exception.Message)".Contains("OCR_UNAVAILABLE")) {
    $saida = [ordered]@{ ok = $false; motivo = "ocr_indisponivel"; imagem = $Imagem; palavras = @() }
    $jsonTexto = $saida | ConvertTo-Json -Compress
    Write-Output $jsonTexto
    exit 2
  }
  throw
} finally {
  Remove-Item -Path $tmp -Force -ErrorAction SilentlyContinue
}

$saida = [ordered]@{
  ok       = $true
  imagem   = $Imagem
  regiao   = @($x, $y, $w, $h)
  escala   = $Escala
  palavras = $palavras
}
$jsonTexto = $saida | ConvertTo-Json -Compress -Depth 5
if ($Json -ne "") {
  $destino = if ([System.IO.Path]::IsPathRooted($Json)) { $Json } else { Join-Path $raiz $Json }
  New-Item -ItemType Directory -Force -Path (Split-Path $destino) | Out-Null
  Set-Content -Path $destino -Value $jsonTexto -Encoding utf8
}
Write-Output $jsonTexto
exit 0
