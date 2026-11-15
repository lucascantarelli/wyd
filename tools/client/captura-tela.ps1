# Captura a tela inteira em intervalo fixo, para registrar o que o cliente desenhou quando o
# `PrintWindow` do harness devolve preto (D3D9 não expõe o backbuffer por WM_PRINT).
#
# Uso (não precisa elevar; o alvo é a tela, não o processo do cliente):
#   powershell -NoProfile -ExecutionPolicy Bypass -File tools/client/captura-tela.ps1 `
#     -Saida target/tela -IntervaloMs 400 -DuracaoSegundos 60
param(
  [string]$Saida = "target/tela",
  [int]$IntervaloMs = 400,
  [int]$DuracaoSegundos = 60,
  [int]$Largura = 806,
  [int]$Altura = 629
)

$ErrorActionPreference = "Continue"
$raiz = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$pasta = Join-Path $raiz $Saida
New-Item -ItemType Directory -Force -Path $pasta | Out-Null
Add-Type -AssemblyName System.Drawing

$fim = (Get-Date).AddSeconds($DuracaoSegundos)
$indice = 0
while ((Get-Date) -lt $fim) {
  $arquivo = Join-Path $pasta ("tela-{0:D4}.jpg" -f $indice)
  try {
    $bmp = New-Object System.Drawing.Bitmap($Largura, $Altura)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.CopyFromScreen(0, 0, 0, 0, $bmp.Size)
    $codec = [System.Drawing.Imaging.ImageCodecInfo]::GetImageEncoders() | Where-Object { $_.MimeType -eq "image/jpeg" }
    $par = New-Object System.Drawing.Imaging.EncoderParameters(1)
    $par.Param[0] = New-Object System.Drawing.Imaging.EncoderParameter([System.Drawing.Imaging.Encoder]::Quality, 70)
    $bmp.Save($arquivo, $codec, $par)
    $g.Dispose(); $bmp.Dispose()
  } catch {
    Write-Output "captura $indice falhou: $($_.Exception.Message)"
  }
  $indice += 1
  Start-Sleep -Milliseconds $IntervaloMs
}
Write-Output "capturas: $indice em $pasta"
