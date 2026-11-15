# Reconhece o teclado de PIN do 7662 e devolve o centro de cada dígito, para o harness clicar.
#
# Por que existe: o diálogo "Insira Senha(2)" sorteia a posição dos dígitos a cada sessão
# (medido em 2026-11-15: sessão A = `1 3 9 6 4 / 7 5 2 0 8`, sessão B = `5 0 6 8 7 / 3 4 2 1 9`)
# e o `Windows.Media.Ocr` não lê esses glifos. Clicar por coordenada fixa digitaria o PIN errado
# — e cinco PINs errados bloqueiam a conta por 10 min (`accounts.pin_failures`).
#
# Como funciona: o layout das dez células é fixo (5x2); só o dígito de cada célula muda. O glifo
# é branco (~#ffffff) sobre botão escuro, tem ~7x8 px e é o **mesmo bitmap** em qualquer sessão
# (medido: o "1" sai idêntico em células diferentes, deslocado). A comparação é de máscara:
# pixels claros da janela → caixa do glifo → amostragem numa grade 6x8 (48 bits) → distância de
# Hamming contra as dez referências. A normalização pela caixa elimina o deslocamento e o fundo
# (textura do botão e ornamento estrelado, que mudam de célula para célula) fica fora da máscara.
#
# Tempo: precisa caber nos **10 s** de `HANDSHAKE_TIMEOUT` do canal, então a imagem é lida de uma
# vez (`LockBits` + `Marshal.Copy`) em vez de `GetPixel` por amostra.
#
# Uso:
#   powershell -File tools/client/teclado-pin.ps1 -Imagem target/x.png -GerarRotulos 5,0,6,8,7,3,4,2,1,9
#   powershell -File tools/client/teclado-pin.ps1 -Imagem target/x.png -Json target/teclado.json
param(
  [Parameter(Mandatory = $true)][string]$Imagem,
  [string]$Json = "",
  # Gera as referências a partir desta imagem, na ordem das dez células
  # (linha de cima da esquerda para a direita, depois a de baixo).
  [string]$GerarRotulos = "",
  [string]$ArquivoTemplates = "target/teclado-templates.json",
  # Luminância do glifo: o dígito é branco; o botão e o ornamento ficam abaixo disso.
  [int]$Limiar = 190,
  # Meia-janela em volta do centro da célula, em pixels.
  [int]$MeiaJanela = 10,
  # Grade da máscara normalizada.
  [int]$Colunas = 6,
  [int]$Linhas = 8
)

$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing
$raiz = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$origem = if ([System.IO.Path]::IsPathRooted($Imagem)) { $Imagem } else { Join-Path $raiz $Imagem }
$destinoTemplates = if ([System.IO.Path]::IsPathRooted($ArquivoTemplates)) { $ArquivoTemplates } else { Join-Path $raiz $ArquivoTemplates }

# Centros das dez células no diálogo de PIN (janela 806x629), medidos na captura de 2026-11-15.
$celulas = @(
  @(293, 305), @(352, 305), @(404, 305), @(457, 305), @(510, 305),
  @(293, 351), @(352, 351), @(404, 351), @(457, 351), @(510, 351)
)

# --- leitura da imagem inteira de uma vez ------------------------------------------------
$img = [System.Drawing.Bitmap]::FromFile($origem)
try {
  $script:Largura = $img.Width
  $script:Altura = $img.Height
  $retangulo = New-Object System.Drawing.Rectangle 0, 0, $img.Width, $img.Height
  $dados = $img.LockBits($retangulo, [System.Drawing.Imaging.ImageLockMode]::ReadOnly,
    [System.Drawing.Imaging.PixelFormat]::Format24bppRgb)
  $script:Stride = $dados.Stride
  $script:Pixels = New-Object byte[] ($dados.Stride * $img.Height)
  [System.Runtime.InteropServices.Marshal]::Copy($dados.Scan0, $script:Pixels, 0, $script:Pixels.Length)
  $img.UnlockBits($dados)
} finally { $img.Dispose() }

function Lum([int]$x, [int]$y) {
  $x = [Math]::Max(0, [Math]::Min($x, $script:Largura - 1))
  $y = [Math]::Max(0, [Math]::Min($y, $script:Altura - 1))
  $off = $y * $script:Stride + $x * 3
  # memória 24bpp é BGR
  return 0.114 * $script:Pixels[$off] + 0.587 * $script:Pixels[$off + 1] + 0.299 * $script:Pixels[$off + 2]
}

# Máscara normalizada do glifo na célula: `$null` se não houver pixel claro na janela.
function MascaraCelula([int]$cx, [int]$cy) {
  $x0 = $cx - $MeiaJanela; $y0 = $cy - $MeiaJanela
  $lado = $MeiaJanela * 2
  $brilho = New-Object 'bool[,]' $lado, $lado
  $minX = $lado; $maxX = -1; $minY = $lado; $maxY = -1
  for ($y = 0; $y -lt $lado; $y++) {
    for ($x = 0; $x -lt $lado; $x++) {
      $claro = (Lum ($x0 + $x) ($y0 + $y)) -ge $Limiar
      $brilho[$x, $y] = $claro
      if ($claro) {
        if ($x -lt $minX) { $minX = $x }
        if ($x -gt $maxX) { $maxX = $x }
        if ($y -lt $minY) { $minY = $y }
        if ($y -gt $maxY) { $maxY = $y }
      }
    }
  }
  if ($maxX -lt 0) { return $null }

  $larg = $maxX - $minX + 1
  $alt = $maxY - $minY + 1
  $bits = ""
  for ($ly = 0; $ly -lt $Linhas; $ly++) {
    for ($lx = 0; $lx -lt $Colunas; $lx++) {
      # amostra o centro da sub-célula da caixa do glifo
      $sx = $minX + [int][Math]::Floor(($lx + 0.5) * $larg / $Colunas)
      $sy = $minY + [int][Math]::Floor(($ly + 0.5) * $alt / $Linhas)
      $sx = [Math]::Max($minX, [Math]::Min($sx, $maxX))
      $sy = [Math]::Max($minY, [Math]::Min($sy, $maxY))
      $bits += $(if ($brilho[$sx, $sy]) { "1" } else { "0" })
    }
  }
  return [ordered]@{ bits = $bits; largura = $larg; altura = $alt }
}

function Hamming([string]$a, [string]$b) {
  $d = 0
  for ($i = 0; $i -lt $a.Length; $i++) { if ($a[$i] -ne $b[$i]) { $d++ } }
  return $d
}

if ($GerarRotulos -ne "") {
  $rotulos = @($GerarRotulos.Split(",") | ForEach-Object { $_.Trim() })
  if ($rotulos.Count -ne 10) { throw "-GerarRotulos pede os dez dígitos na ordem das células" }
  $templates = [ordered]@{}
  for ($i = 0; $i -lt 10; $i++) {
    $m = MascaraCelula $celulas[$i][0] $celulas[$i][1]
    if (-not $m) { throw "célula $i sem glifo (limiar $Limiar)" }
    $templates[$rotulos[$i]] = $m
  }
  New-Item -ItemType Directory -Force -Path (Split-Path $destinoTemplates) | Out-Null
  ($templates | ConvertTo-Json -Compress -Depth 5) | Set-Content -Path $destinoTemplates -Encoding utf8
  Write-Output "referências em $destinoTemplates : $($rotulos -join ',')"
  exit 0
}

if (-not (Test-Path $destinoTemplates)) { throw "sem referências em $destinoTemplates (rode -GerarRotulos)" }
$templates = Get-Content -Path $destinoTemplates -Raw | ConvertFrom-Json
$nomes = @($templates.PSObject.Properties.Name)
if ($nomes.Count -lt 10) { throw "referências incompletas: $($nomes.Count)/10" }

$digitos = [ordered]@{}
$layout = @()
$distancias = @()
for ($i = 0; $i -lt 10; $i++) {
  $cx = $celulas[$i][0]; $cy = $celulas[$i][1]
  $m = MascaraCelula $cx $cy
  if (-not $m) { throw "célula $i sem glifo na captura (limiar $Limiar)" }
  $melhor = $null; $melhorD = 999
  foreach ($nome in $nomes) {
    $d = Hamming $m.bits $templates.$nome.bits
    if ($d -lt $melhorD) { $melhorD = $d; $melhor = $nome }
  }
  $layout += $melhor
  $distancias += $melhorD
  $digitos[$melhor] = [ordered]@{ celula = $i; x = $cx; y = $cy; distancia = $melhorD; bits = $m.bits }
}

$saida = [ordered]@{
  ok         = (($digitos.Count -eq 10) -and (($distancias | Measure-Object -Maximum).Maximum -le 6))
  imagem     = $Imagem
  grade      = "$Colunas x $Linhas"
  layout     = @($layout)
  distancias = @($distancias)
  digitos    = $digitos
}
$jsonTexto = $saida | ConvertTo-Json -Compress -Depth 6
if ($Json -ne "") {
  $alvo = if ([System.IO.Path]::IsPathRooted($Json)) { $Json } else { Join-Path $raiz $Json }
  New-Item -ItemType Directory -Force -Path (Split-Path $alvo) | Out-Null
  Set-Content -Path $alvo -Value $jsonTexto -Encoding utf8
}
Write-Output $jsonTexto
