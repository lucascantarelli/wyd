# Sessão de captura do cliente 7662 (FASE-08-6) — exige elevação.
#
# Por que elevação: `client/WYD.exe` declara `requireAdministrator` no manifesto, então os processos
# do cliente não podem ser encerrados por um shell comum e mouse/teclado de processo não elevado não
# atravessa a UIPI.
#
# Fluxo (medido em 2026-11-14):
#   - `client/serverlist.bin` aponta para 192.168.100.3/.4, que já existem como endereços desta
#     máquina;
#   - ao abrir, o cliente faz `GET /serv00.htm` na porta 80 para preencher a coluna Canal — se o
#     servidor Rust deve responder, use `-SemListeners`, senão o listener local deste harness segura
#     a requisição e a coluna fica vazia;
#   - o TCP 8281 só abre depois de escolher o servidor, o canal e clicar em "Conectar"; a tela é
#     D3D9, sem controles Win32 para UIAutomation.
#
# ENTRADA: `SetCursorPos` + `mouse_event` **não chega** a este cliente (medido: nem o botão "Fechar"
# encerra o processo). O que chega é `SendInput` com `MOUSEEVENTF_ABSOLUTE` para o mouse e
# `KEYEVENTF_SCANCODE` para o teclado — é o caminho usado aqui em todas as etapas.
#
# CAPTURA: `PrintWindow` de janela D3D9 sai preto; a captura é `CopyFromScreen` (o que está de fato
# na tela).
#
# Este script: encerra clientes antigos, sobe listeners (8281 + 80) salvo `-SemListeners`, abre o
# cliente, mede janela/DPI/foreground, escolhe servidor e canal, clica em Conectar, e — se `-Conta`
# for informada — digita conta/senha no LOGIN e captura a resposta desenhada (a recusa dura ~4 s:
# `push 0xfa0` em `0x4b40b7`).
param(
  [int]$Porta = 8281,
  [int]$EsperaSegundos = 110,
  [string]$Artefato = "target/captura-8281.json",
  [string]$Log = "target/captura-sessao.log",
  # Coordenadas da janela 806x629 em 0,0 (a janela real desta máquina): linha "WYD" da coluna
  # Servidor, primeira linha ("WYD-1") da coluna Canal e botão Conectar.
  [int]$ServidorX = 330,
  [int]$ServidorY = 318,
  [int]$CanalX = 465,
  [int]$CanalY = 318,
  [int]$ConectarX = 330,
  [int]$ConectarY = 504,
  # Tela de Login (mesma janela): campo ID, campo Senha.
  [int]$LoginIdX = 434,
  [int]$LoginIdY = 293,
  [int]$LoginSenhaX = 434,
  [int]$LoginSenhaY = 318,
  # Não sobe listener local: o canal do compose responde 8281 e o `serv00.htm` da porta 80.
  [switch]$SemListeners,
  # Segundos entre o Enter do login e o screenshot da mensagem de recusa.
  [double]$EsperaMensagem = 1.2,
  # Se informados, digita conta/senha e envia o login, capturando a resposta.
  [string]$Conta = "",
  [string]$Senha = "",
  [string]$Prefixo = "login"
)

$ErrorActionPreference = "Continue"
$raiz = (Resolve-Path (Join-Path $PSScriptRoot "..\..")).Path
$logPath = Join-Path $raiz $Log
New-Item -ItemType Directory -Force -Path (Split-Path $logPath) | Out-Null
function Escreve([string]$texto) {
  $linha = "$(Get-Date -Format o) $texto"
  Write-Host $linha
  Add-Content -Path $logPath -Value $linha -Encoding utf8
}

Add-Type -AssemblyName System.Drawing
Add-Type -TypeDefinition @"
using System;
using System.Drawing;
using System.Runtime.InteropServices;
public class TelaCliente {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left, Top, Right, Bottom; }
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [StructLayout(LayoutKind.Sequential)] public struct MOUSEINPUT { public int dx, dy; public uint mouseData, dwFlags, time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Sequential)] public struct KEYBDINPUT { public ushort wVk, wScan; public uint dwFlags, time; public IntPtr dwExtraInfo; }
  [StructLayout(LayoutKind.Explicit)] public struct INPUTUNION { [FieldOffset(0)] public MOUSEINPUT mi; [FieldOffset(0)] public KEYBDINPUT ki; }
  [StructLayout(LayoutKind.Sequential)] public struct INPUT { public uint type; public INPUTUNION u; }

  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out POINT p);
  [DllImport("user32.dll")] public static extern uint SendInput(uint n, INPUT[] p, int cb);
  [DllImport("user32.dll")] public static extern int GetSystemMetrics(int i);
  [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern ushort MapVirtualKey(uint code, uint map);

  public static RECT Retangulo(IntPtr hwnd) { RECT r; GetWindowRect(hwnd, out r); return r; }
  public static POINT Cursor() { POINT p; GetCursorPos(out p); return p; }

  // clique absoluto normalizado (0..65535) — o caminho que este cliente aceita
  public static uint Clique(int x, int y) {
    int w = GetSystemMetrics(0), h = GetSystemMetrics(1);
    var entrada = new INPUT[2];
    entrada[0].type = 0;
    entrada[0].u.mi.dx = (int)Math.Round(x * 65535.0 / Math.Max(w - 1, 1));
    entrada[0].u.mi.dy = (int)Math.Round(y * 65535.0 / Math.Max(h - 1, 1));
    entrada[0].u.mi.dwFlags = 0x8001; // MOVE | ABSOLUTE
    entrada[1] = entrada[0];
    entrada[1].u.mi.dwFlags = 0x8002; // DOWN | ABSOLUTE
    SendInput(2, entrada, Marshal.SizeOf(typeof(INPUT)));
    System.Threading.Thread.Sleep(60);
    var solta = new INPUT[1];
    solta[0] = entrada[1];
    solta[0].u.mi.dwFlags = 0x8004;   // UP | ABSOLUTE
    return SendInput(1, solta, Marshal.SizeOf(typeof(INPUT)));
  }

  public static uint Tecla(ushort vk) {
    ushort scan = MapVirtualKey(vk, 0);
    var par = new INPUT[2];
    par[0].type = 1; par[0].u.ki.wVk = 0; par[0].u.ki.wScan = scan; par[0].u.ki.dwFlags = 0x0008;
    par[1].type = 1; par[1].u.ki.wVk = 0; par[1].u.ki.wScan = scan; par[1].u.ki.dwFlags = 0x0008 | 0x0002;
    return SendInput(2, par, Marshal.SizeOf(typeof(INPUT)));
  }

  // digita por scan code (o teclado do cliente é lido via DirectInput: WM_CHAR não alimenta)
  public static int Texto(string texto) {
    int enviados = 0;
    foreach (char c in texto) {
      ushort vk;
      if (c >= 'a' && c <= 'z') vk = (ushort)('A' + (c - 'a'));
      else if (c >= 'A' && c <= 'Z') vk = c;
      else if (c >= '0' && c <= '9') vk = c;
      else continue;
      enviados += (int)Tecla(vk);
      System.Threading.Thread.Sleep(45);
    }
    return enviados;
  }

  public static void Print(IntPtr hwnd, string caminho) {
    RECT r = Retangulo(hwnd);
    int w = Math.Max(r.Right - r.Left, 1), h = Math.Max(r.Bottom - r.Top, 1);
    using (var bmp = new Bitmap(w, h))
    using (var g = Graphics.FromImage(bmp)) {
      IntPtr hdc = g.GetHdc();
      PrintWindow(hwnd, hdc, 2);
      g.ReleaseHdc(hdc);
      bmp.Save(caminho, System.Drawing.Imaging.ImageFormat.Png);
    }
  }
  [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr hwnd, IntPtr hdc, uint flags);

  // o que está de fato na tela (PrintWindow de D3D9 volta preto)
  public static void Captura(string caminho, int x, int y, int w, int h) {
    using (var bmp = new Bitmap(Math.Max(w, 1), Math.Max(h, 1)))
    using (var g = Graphics.FromImage(bmp)) {
      g.CopyFromScreen(x, y, 0, 0, bmp.Size);
      bmp.Save(caminho, System.Drawing.Imaging.ImageFormat.Png);
    }
  }
}
"@ -ReferencedAssemblies System.Drawing

[void][TelaCliente]::SetProcessDPIAware()

# captura a janela em PNG (tela real) + JPEG 700px para leitura no relatório
function Screenshot([IntPtr]$handle, [string]$nome) {
  $r = [TelaCliente]::Retangulo($handle)
  $larg = $r.Right - $r.Left; $alt = $r.Bottom - $r.Top
  $png = Join-Path $raiz "target/$nome.png"
  try {
    [TelaCliente]::Captura($png, $r.Left, $r.Top, $larg, $alt)
    Escreve "screenshot: $png"
  } catch { Escreve "screenshot $nome falhou: $($_.Exception.Message)" }
  try {
    $img = [System.Drawing.Image]::FromFile($png)
    $l = 700
    $a = [int]($img.Height * ($l / $img.Width))
    $bmp = New-Object System.Drawing.Bitmap($l, $a)
    $g = [System.Drawing.Graphics]::FromImage($bmp)
    $g.InterpolationMode = "HighQualityBicubic"
    $g.DrawImage($img, 0, 0, $l, $a)
    $codec = [System.Drawing.Imaging.ImageCodecInfo]::GetImageEncoders() | Where-Object { $_.MimeType -eq "image/jpeg" }
    $par = New-Object System.Drawing.Imaging.EncoderParameters(1)
    $par.Param[0] = New-Object System.Drawing.Imaging.EncoderParameter([System.Drawing.Imaging.Encoder]::Quality, 70)
    $bmp.Save((Join-Path $raiz "target/$nome.jpg"), $codec, $par)
    $g.Dispose(); $bmp.Dispose(); $img.Dispose()
  } catch { Escreve "jpeg $nome falhou: $($_.Exception.Message)" }
}

function Clique([int]$x, [int]$y) {
  [TelaCliente]::Clique($x, $y) | Out-Null
  Start-Sleep -Milliseconds 250
}

Escreve "=== inicio: raiz=$raiz porta=$Porta espera=$EsperaSegundos ===$([Environment]::NewLine)elevacao: $(([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
Escreve ("tela: " + ([TelaCliente]::GetSystemMetrics(0)) + "x" + ([TelaCliente]::GetSystemMetrics(1)))

Get-Process WYD -ErrorAction SilentlyContinue | ForEach-Object {
  Escreve "encerrando WYD.exe antigo PID=$($_.Id)"
  $_ | Stop-Process -Force -ErrorAction Continue
}
Get-CimInstance Win32_Process -Filter "Name='node.exe'" -ErrorAction SilentlyContinue |
  Where-Object { $_.CommandLine -like "*captura-live.mjs*" } |
  ForEach-Object {
    Escreve "encerrando listener antigo PID=$($_.ProcessId)"
    Stop-Process -Id $_.ProcessId -Force -ErrorAction Continue
  }
Start-Sleep -Seconds 2

foreach ($ip in @("192.168.100.3", "192.168.100.4")) {
  $saida = (& netsh interface ipv4 add address "Loopback Pseudo-Interface 1" $ip 255.255.255.255 2>&1 | Out-String).Trim()
  Escreve "netsh add $ip -> $saida"
}

$nodeExe = (Get-Command node).Source
$outLog = Join-Path $raiz "target/captura-live.out.log"
$errLog = Join-Path $raiz "target/captura-live.err.log"
if ($SemListeners) {
  Escreve "SemListeners: nenhum listener local; o cliente fala com o canal do compose (8281/:80)"
  $node = $null
  $node80 = $null
} else {
  $node = Start-Process -FilePath $nodeExe -ArgumentList @("tools/client/captura-live.mjs", "--host", "0.0.0.0", "--port", "$Porta", "--timeout", "$($EsperaSegundos * 1000)", "--out", $Artefato) -WorkingDirectory $raiz -PassThru -WindowStyle Hidden -RedirectStandardOutput $outLog -RedirectStandardError $errLog
  Escreve "listener $Porta : PID=$($node.Id)"
  $node80 = Start-Process -FilePath $nodeExe -ArgumentList @("tools/client/captura-live.mjs", "--host", "0.0.0.0", "--port", "80", "--timeout", "$($EsperaSegundos * 1000)", "--out", "target/captura-http80.json") -WorkingDirectory $raiz -PassThru -WindowStyle Hidden -RedirectStandardOutput (Join-Path $raiz "target/captura-live80.out.log") -RedirectStandardError (Join-Path $raiz "target/captura-live80.err.log")
  Escreve "listener 80 : PID=$($node80.Id)"
}
Start-Sleep -Seconds 3

$cli = Start-Process -FilePath (Join-Path $raiz "client/WYD.exe") -WorkingDirectory (Join-Path $raiz "client") -PassThru
Escreve "cliente: PID=$($cli.Id)"
Start-Sleep -Seconds 30

$proc = Get-Process -Id $cli.Id -ErrorAction SilentlyContinue
if (-not $proc) {
  Escreve "cliente morreu antes da automação"
} else {
  $handle = $proc.MainWindowHandle
  $r = [TelaCliente]::Retangulo($handle)
  $larg = $r.Right - $r.Left
  $alt = $r.Bottom - $r.Top
  Escreve "janela: '$($proc.MainWindowTitle)' handle=$handle rect=$($r.Left),$($r.Top) ${larg}x${alt} dpi=$([TelaCliente]::GetDpiForWindow($handle))"
  [TelaCliente]::SetForegroundWindow($handle) | Out-Null
  Start-Sleep -Milliseconds 500
  Escreve "foreground=$([TelaCliente]::GetForegroundWindow()) igual_ao_cliente=$([TelaCliente]::GetForegroundWindow() -eq $handle)"
  Screenshot $handle "etapa1-menu"

  Escreve "clique no servidor ($ServidorX,$ServidorY)"
  Clique ($r.Left + $ServidorX) ($r.Top + $ServidorY)
  Start-Sleep -Seconds 2
  Screenshot $handle "etapa2-servidor"

  Escreve "clique no canal ($CanalX,$CanalY)"
  Clique ($r.Left + $CanalX) ($r.Top + $CanalY)
  Start-Sleep -Seconds 2
  Screenshot $handle "etapa3-canal"

  # navegação por scan code na lista (seta para baixo + Enter), como no caminho medido
  foreach ($vk in @(0x28, 0x28, 0x0D)) {
    [TelaCliente]::Tecla([uint16]$vk) | Out-Null
    Start-Sleep -Milliseconds 400
  }

  Escreve "clique em Conectar ($ConectarX,$ConectarY)"
  Clique ($r.Left + $ConectarX) ($r.Top + $ConectarY)
  Start-Sleep -Seconds 6
  Screenshot $handle "etapa4-conectar"

  $estabelecidas = Get-NetTCPConnection -LocalPort $Porta -State Established -ErrorAction SilentlyContinue
  Escreve "conexoes ESTABELECIDAS na porta ${Porta}: $(($estabelecidas | Measure-Object).Count)"

  if ($Conta -ne "") {
    $ix = $r.Left + $LoginIdX; $iy = $r.Top + $LoginIdY
    Escreve "clique no campo ID ($ix,$iy)"
    Clique $ix $iy
    Start-Sleep -Milliseconds 300
    Escreve "digita conta '$Conta' eventos=$([TelaCliente]::Texto($Conta))"
    Start-Sleep -Milliseconds 250
    [TelaCliente]::Tecla(0x09) | Out-Null   # TAB -> senha
    Start-Sleep -Milliseconds 250
    Escreve "digita senha ($($Senha.Length) caracteres) eventos=$([TelaCliente]::Texto($Senha))"
    Start-Sleep -Milliseconds 300
    Screenshot $handle "$Prefixo-preenchido"
    Escreve "Enter no login"
    [TelaCliente]::Tecla(0x0D) | Out-Null
    Start-Sleep -Milliseconds ([int]($EsperaMensagem * 1000))
    Screenshot $handle "$Prefixo-recusa-1"
    Escreve "captura 1: target/$Prefixo-recusa-1.jpg"
    Start-Sleep -Milliseconds 1800
    Screenshot $handle "$Prefixo-recusa-2"
    Escreve "captura 2: target/$Prefixo-recusa-2.jpg"
    $est2 = Get-NetTCPConnection -LocalPort $Porta -State Established -ErrorAction SilentlyContinue
    Escreve "conexoes ESTABELECIDAS na porta ${Porta} no login: $(($est2 | Measure-Object).Count)"
  }
}

if ($node) {
  $node.WaitForExit($EsperaSegundos * 1000 + 15000) | Out-Null
  if (-not $node.HasExited) { $node | Stop-Process -Force -ErrorAction Continue }
}
if ($node80) {
  $node80.WaitForExit($EsperaSegundos * 1000 + 15000) | Out-Null
  if (-not $node80.HasExited) { $node80 | Stop-Process -Force -ErrorAction Continue }
}
Get-Process WYD -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction Continue
Escreve "cliente encerrado"

foreach ($arquivo in @($outLog, $errLog, (Join-Path $raiz "target/captura-live80.out.log"), (Join-Path $raiz "client/WYD.log"))) {
  if (Test-Path $arquivo) { Escreve ("--- $arquivo`n" + (Get-Content $arquivo -Raw)) }
}
Escreve "=== fim ==="
