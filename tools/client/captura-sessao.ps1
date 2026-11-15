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
  [string]$Prefixo = "login",
  # Comparação automática da mensagem desenhada com o registro do `UI/strdef.bin`
  # (12 = "Usuário ou senha incorretos.", o único literal da tela de login; era o texto de
  # fábrica "Conexão anterior finalizada. Tente novamente." até `strdef-patch.mjs` reescrevê-lo).
  # `-StrdefMensagem -1` desliga a leitura; `-OcrMinimo` é a semelhança mínima normalizada.
  [int]$StrdefMensagem = 12,
  [double]$OcrMinimo = 0.85,
  [int]$OcrFatias = 10,
  # Segunda senha (PIN) do 7662. Depois do login aceito o cliente abre o diálogo
  # "Insira Senha(2)" — um teclado 5x2 com os dígitos desenhados (`1 3 9 6 4 / 7 5 2 0 8` na
  # captura de 2026-11-15) e os botões Confirmar/Cancelar/Alterar. Aqui o PIN é **digitado por
  # scan code** (mesmo caminho do login); se o diálogo só aceitar clique, o harness cai nas
  # coordenadas medidas do teclado (`-TecladoPorClique`).
  [string]$Pin = "",
  [switch]$TecladoPorClique,
  # Texto esperado do painel `0x0101` quando o PIN é recusado ("PIN incorreto. Tente novamente."
  # ou "Senha numerica exigida. Tente novamente."); vazio = não compara.
  [string]$PainelEsperado = "",
  # Entrada no mapa: duplo clique no personagem do slot (mesma janela) e espera da cena.
  [int]$PersonagemX = 0,
  [int]$PersonagemY = 0,
  # Fluxo de **troca** da segunda senha pela tela do cliente (botão "Alterar"): com
  # `-ModoTroca`, `-Pin` é o PIN atual, `-PinNovo` o novo, e cada etapa clica em Alterar.
  [switch]$ModoTroca,
  [string]$PinNovo = "",
  [int]$AlterarX = 497,
  [int]$AlterarY = 406,
  # Botão "Conectar" do painel do personagem (aberto pelo duplo clique na seleção).
  [int]$PainelConectarX = 570,
  [int]$PainelConectarY = 408,
  # Espera entre o PIN aceito e a captura da seleção (o seletor desenha o personagem
  # depois do `0x0FDE`; 3 s bastam no caminho feliz, mais quando a máquina está ocupada).
  [int]$EsperaSelecao = 3,
  [int]$EsperaMapa = 8
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
using System.Drawing.Imaging;
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

  // --- teclado de PIN -------------------------------------------------------------------
  // Os dígitos do diálogo "Insira Senha(2)" mudam de posição a cada sessão e o OCR não os lê;
  // a posição sai da máscara do glifo (48 bits, caixa normalizada) comparada com as referências
  // medidas por `tools/client/teclado-pin.ps1`. Aqui a comparação é **no processo do harness**:
  // o canal fecha o handshake em 10 s, e um `powershell` filho custa ~8 s só de startup.
  public static string ReconheceTeclado(string caminho, string referencias, int limiar) {
    var refs = new System.Collections.Generic.Dictionary<string, string>();
    foreach (var par in referencias.Split(';')) {
      int corte = par.IndexOf(':');
      if (corte > 0) refs[par.Substring(0, corte)] = par.Substring(corte + 1);
    }
    int[] xs = { 293, 352, 404, 457, 510, 293, 352, 404, 457, 510 };
    int[] ys = { 305, 305, 305, 305, 305, 351, 351, 351, 351, 351 };
    using (var bmp = new Bitmap(caminho)) {
      var dados = bmp.LockBits(new Rectangle(0, 0, bmp.Width, bmp.Height),
        ImageLockMode.ReadOnly, PixelFormat.Format24bppRgb);
      var bytes = new byte[dados.Stride * bmp.Height];
      Marshal.Copy(dados.Scan0, bytes, 0, bytes.Length);
      bmp.UnlockBits(dados);
      var saida = new System.Text.StringBuilder();
      for (int c = 0; c < 10; c++) {
        string bits = MascaraGlifo(bytes, dados.Stride, bmp.Width, bmp.Height, xs[c], ys[c], limiar);
        string melhor = "?"; int melhorD = 999;
        foreach (var kv in refs) {
          int d = Distancia(bits, kv.Value);
          if (d < melhorD) { melhorD = d; melhor = kv.Key; }
        }
        if (c > 0) saida.Append(';');
        saida.Append(melhor).Append(':').Append(xs[c]).Append(':').Append(ys[c]).Append(':').Append(melhorD);
      }
      return saida.ToString();
    }
  }

  static string MascaraGlifo(byte[] px, int stride, int w, int h, int cx, int cy, int limiar) {
    const int lado = 20, meia = 10;
    bool[,] claro = new bool[lado, lado];
    int minX = lado, maxX = -1, minY = lado, maxY = -1;
    for (int y = 0; y < lado; y++) {
      for (int x = 0; x < lado; x++) {
        int sx = Math.Min(Math.Max(cx - meia + x, 0), w - 1);
        int sy = Math.Min(Math.Max(cy - meia + y, 0), h - 1);
        int off = sy * stride + sx * 3; // 24bpp BGR
        double lum = 0.114 * px[off] + 0.587 * px[off + 1] + 0.299 * px[off + 2];
        bool b = lum >= limiar;
        claro[x, y] = b;
        if (b) {
          if (x < minX) minX = x;
          if (x > maxX) maxX = x;
          if (y < minY) minY = y;
          if (y > maxY) maxY = y;
        }
      }
    }
    if (maxX < 0) return "";
    int larg = maxX - minX + 1, alt = maxY - minY + 1;
    var sb = new System.Text.StringBuilder(48);
    for (int ly = 0; ly < 8; ly++) {
      for (int lx = 0; lx < 6; lx++) {
        int sx = minX + (int)Math.Floor((lx + 0.5) * larg / 6.0);
        int sy = minY + (int)Math.Floor((ly + 0.5) * alt / 8.0);
        sx = Math.Min(Math.Max(sx, minX), maxX);
        sy = Math.Min(Math.Max(sy, minY), maxY);
        sb.Append(claro[sx, sy] ? '1' : '0');
      }
    }
    return sb.ToString();
  }

  static int Distancia(string a, string b) {
    if (a.Length != b.Length) return 999;
    int d = 0;
    for (int i = 0; i < a.Length; i++) { if (a[i] != b[i]) d++; }
    return d;
  }

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

# Captura só o PNG (sem o JPEG do relatório): é a que cabe no prazo do handshake.
function ScreenshotRapido([IntPtr]$handle, [string]$nome) {
  $rect = [TelaCliente]::Retangulo($handle)
  $png = Join-Path $raiz "target/$nome.png"
  [TelaCliente]::Captura($png, $rect.Left, $rect.Top, ($rect.Right - $rect.Left), ($rect.Bottom - $rect.Top))
  return $png
}

function Clique([int]$x, [int]$y) {
  [TelaCliente]::Clique($x, $y) | Out-Null
  Start-Sleep -Milliseconds 250
}

# A lista de canais vem por HTTP (`GET /serv00.htm` na porta 80) e pode demorar: esperar um
# tempo fixo e clicar em Conectar dava "coluna Canal vazia" e o cliente ficava na tela de
# servidor (medido em 2026-11-15). Aqui a espera é do **fato** — conexão TCP estabelecida —
# com prazo, e quem chama repete a sequência se ela não veio.
function AguardarConexao([int]$segundos) {
  $limite = (Get-Date).AddSeconds($segundos)
  while ((Get-Date) -lt $limite) {
    $c = Get-NetTCPConnection -RemotePort $Porta -State Established -ErrorAction SilentlyContinue
    if ($c) { return $c }
    Start-Sleep -Milliseconds 300
  }
  return $null
}

# Texto desenhado na captura, sem comparação: serve para saber **em que tela** o cliente está
# (a tela de login traz "ID"/"Senha" e só aparece depois que Conectar abre o socket).
function TextoDaTela([string]$imagem) {
  $saida = (& powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $raiz "tools/client/ocr-texto.ps1") `
      -Imagem $imagem -Strdef -1 -Escala 4 -Fatias 10 | Out-String).Trim()
  try { return ($saida | ConvertFrom-Json).lido } catch { return "" }
}

# Leitura automática da mensagem desenhada: OCR offline (Windows.Media.Ocr) + comparação com o
# literal do registro `UI/strdef.bin` feito por `tools/client/ocr-texto.ps1`. Devolve o objeto
# JSON e grava o artefato; `ok=false`/`exit 1` significa que a captura divergiu do esperado.
function OcrMensagem([string]$imagem, [string]$artefato) {
  $script = Join-Path $raiz "tools/client/ocr-texto.ps1"
  $saida = (& powershell -NoProfile -ExecutionPolicy Bypass -File $script -Imagem $imagem `
      -Strdef $StrdefMensagem -Escala 4 -Fatias $OcrFatias -Minimo $OcrMinimo -Json $artefato |
    Out-String).Trim()
  $codigo = $LASTEXITCODE
  Escreve "ocr $imagem -> exit=$codigo $artefato`n$saida"
  try { return ($saida | ConvertFrom-Json) } catch { return $null }
}

Escreve "=== inicio: raiz=$raiz porta=$Porta espera=$EsperaSegundos ===$([Environment]::NewLine)elevacao: $(([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator))"
Escreve ("tela: " + ([TelaCliente]::GetSystemMetrics(0)) + "x" + ([TelaCliente]::GetSystemMetrics(1)))

Get-Process WYD -ErrorAction SilentlyContinue | ForEach-Object {
  Escreve "encerrando WYD.exe antigo PID=$($_.Id)"
  $_ | Stop-Process -Force -ErrorAction Continue
}
# Com `-SemListeners` o harness **não** gerencia listener: quem escuta na 8281 pode ser um
# proxy de captura (`captura-live.mjs --alvo`) que o chamador subiu de propósito — matá-lo
# derrubava a conexão do cliente antes do handshake (medido em 2026-11-15).
if (-not $SemListeners) {
  Get-CimInstance Win32_Process -Filter "Name='node.exe'" -ErrorAction SilentlyContinue |
    Where-Object { $_.CommandLine -like "*captura-live.mjs*" } |
    ForEach-Object {
      Escreve "encerrando listener antigo PID=$($_.ProcessId)"
      Stop-Process -Id $_.ProcessId -Force -ErrorAction Continue
    }
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

  $conexao = $null
  $telaLogin = $false
  for ($tentativa = 1; $tentativa -le 3 -and -not $telaLogin; $tentativa++) {
    Escreve "tentativa $tentativa : servidor ($ServidorX,$ServidorY) -> canal ($CanalX,$CanalY) -> Conectar ($ConectarX,$ConectarY)"
    Clique ($r.Left + $ServidorX) ($r.Top + $ServidorY)
    Start-Sleep -Milliseconds 1200
    if ($tentativa -eq 1) { Screenshot $handle "etapa2-servidor" }
    Clique ($r.Left + $CanalX) ($r.Top + $CanalY)
    Start-Sleep -Milliseconds 1200
    if ($tentativa -eq 1) { Screenshot $handle "etapa3-canal" }
    # navegação por scan code na lista (seta para baixo + Enter), como no caminho medido
    foreach ($vk in @(0x28, 0x28, 0x0D)) {
      [TelaCliente]::Tecla([uint16]$vk) | Out-Null
      Start-Sleep -Milliseconds 350
    }
    Clique ($r.Left + $ConectarX) ($r.Top + $ConectarY)
    Start-Sleep -Seconds 3
    $conexao = AguardarConexao 4
    Screenshot $handle "$Prefixo-login"
    $textoLogin = TextoDaTela "target/$Prefixo-login.png"
    Escreve "tentativa $tentativa : conexao=$([bool]$conexao) tela='$textoLogin'"
    if ($textoLogin -match "(?i)senha|login|voltar|novo") { $telaLogin = $true; break }
  }
  Screenshot $handle "etapa4-conectar"

  # O cliente é quem abre a conexão: a porta 8281 é a **remota** do lado dele.
  $estabelecidas = Get-NetTCPConnection -RemotePort $Porta -State Established -ErrorAction SilentlyContinue
  Escreve "conexoes ESTABELECIDAS com ${Porta}: $(($estabelecidas | Measure-Object).Count) (remoto=$($estabelecidas.RemoteAddress -join ','))"
  if ($Conta -ne "" -and -not $telaLogin) {
    Escreve "tela de login nao apareceu: credenciais nao digitadas (cliente preso na tela de servidor)"
  }

  if ($Conta -ne "" -and $telaLogin) {
    Start-Sleep -Milliseconds 500
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
    if ($Pin -eq "") {
      Start-Sleep -Milliseconds ([int]($EsperaMensagem * 1000))
      Screenshot $handle "$Prefixo-recusa-1"
      Escreve "captura 1: target/$Prefixo-recusa-1.jpg"
      $ocr1 = $null
      if ($StrdefMensagem -ge 0) {
        $ocr1 = OcrMensagem "target/$Prefixo-recusa-1.png" "target/$Prefixo-ocr-1.json"
      }
      Start-Sleep -Milliseconds 1800
      Screenshot $handle "$Prefixo-recusa-2"
      Escreve "captura 2: target/$Prefixo-recusa-2.jpg"
      $ocr2 = $null
      if ($StrdefMensagem -ge 0) {
        $ocr2 = OcrMensagem "target/$Prefixo-recusa-2.png" "target/$Prefixo-ocr-2.json"
      }
      $est2 = Get-NetTCPConnection -RemotePort $Porta -State Established -ErrorAction SilentlyContinue
      Escreve "conexoes ESTABELECIDAS com ${Porta} no login: $(($est2 | Measure-Object).Count)"
      # Veredito único e parseável: alguma captura confere com o registro do strdef.bin.
      $confere = @($ocr1, $ocr2) | Where-Object { $_ -and $_.ok }
      if ($StrdefMensagem -lt 0) {
        Escreve "veredito_ocr=desligado"
      } elseif ($confere) {
        $melhor = $confere | Sort-Object -Property semelhanca -Descending | Select-Object -First 1
        Escreve "veredito_ocr=confere strdef=$StrdefMensagem semelhanca=$($melhor.semelhanca) texto='$($melhor.lido)'"
      } else {
        $lidos = @($ocr1, $ocr2) | Where-Object { $_ } | ForEach-Object { "$($_.semelhanca):'$($_.lido)'" }
        Escreve "veredito_ocr=divergente strdef=$StrdefMensagem leituras=$($lidos -join ' | ')"
      }
    }

    # --- segunda senha (PIN) -------------------------------------------------------------
    if ($Pin -ne "") {
      # O canal fecha o handshake em **10 s** (`HANDSHAKE_TIMEOUT` em `cpsock/mod.rs`): o diálogo
      # é capturado imediatamente e o reconhecimento/clique tem de caber nesse prazo — medido em
      # 2026-11-15, a variante que esperava as duas capturas de recusa antes do PIN levava
      # `pin result=refused reason=pin_ausente kind=WouldBlock`.
      Start-Sleep -Milliseconds 900
      Screenshot $handle "$Prefixo-pin"
      if ($TecladoPorClique) {
        # Os dígitos são desenhados em posições **sorteadas** a cada sessão (medido em
        # 2026-11-15: `1 3 9 6 4 / 7 5 2 0 8` numa sessão e `5 0 6 8 7 / 3 4 2 1 9` na seguinte),
        # então a posição de cada dígito é lida da própria captura por `teclado-pin.ps1`
        # (máscara do glifo normalizada pela caixa) em vez de coordenada fixa.
        # referências medidas por `teclado-pin.ps1` (mesma máscara de 48 bits)
        $arquivoRefs = Join-Path $raiz "target/teclado-templates.json"
        $teclado = @{}
        $distancias = @()
        $layout = @()
        if (-not (Test-Path $arquivoRefs)) {
          Escreve "sem referencias do teclado em $arquivoRefs (rode teclado-pin.ps1 -GerarRotulos): PIN nao digitado"
        } else {
          $tpl = Get-Content $arquivoRefs -Raw | ConvertFrom-Json
          $refsTexto = (@($tpl.PSObject.Properties | ForEach-Object { "$($_.Name):$($_.Value.bits)" }) -join ";")
          # O diálogo pode não estar desenhado no primeiro frame (medido: 0,9 s após o Enter
          # ainda faltavam quatro glifos). Repete a captura até o teclado estar completo.
          for ($tentativaPng = 1; $tentativaPng -le 6; $tentativaPng++) {
            $png = ScreenshotRapido $handle "$Prefixo-pin"
            $bruto = [TelaCliente]::ReconheceTeclado($png, $refsTexto, 190)
            $teclado = @{}; $layout = @(); $distancias = @()
            foreach ($item in $bruto.Split(";")) {
              $partes = $item.Split(":")
              if ($partes.Count -ne 4) { continue }
              $teclado[$partes[0]] = @([int]$partes[1], [int]$partes[2])
              $layout += $partes[0]
              $distancias += [int]$partes[3]
            }
            $pior = ($distancias | Measure-Object -Maximum).Maximum
            Escreve "tentativa $tentativaPng do teclado: layout=$($layout -join ',') pior_distancia=$pior"
            if ($teclado.Count -eq 10 -and $pior -le 6) { break }
            Start-Sleep -Milliseconds 300
          }
        }
        if ($teclado.Count -ne 10 -or ($distancias | Measure-Object -Maximum).Maximum -gt 6) {
          Escreve "teclado nao reconhecido: PIN nao digitado"
        } else {
          $relogio = Get-Date
          foreach ($digito in $Pin.ToCharArray()) {
            $alvo = $teclado["$digito"]
            if (-not $alvo) { Escreve "digito '$digito' fora do teclado"; continue }
            Escreve "clique no digito $digito em ($($alvo[0]),$($alvo[1]))"
            Clique ($r.Left + $alvo[0]) ($r.Top + $alvo[1])
            Start-Sleep -Milliseconds 150
          }
          Escreve "digitos clicados em $([int]((Get-Date) - $relogio).TotalMilliseconds) ms"
          if ($ModoTroca) {
            # Troca pela tela do cliente. Sem `-PinNovo` o harness dá **um** passo (PIN atual +
            # Alterar) para medir o que o cliente faz depois de o servidor aceitar o atual: o
            # campo do diálogo acumula até 6 dígitos e não é limpo entre cliques (medido em
            # 2026-11-15: token "864286" = atual + duas teclas do passo seguinte).
            $etapas = if ($PinNovo -ne "") {
              @(
                @{ pin = $Pin; nome = "troca-atual" },
                @{ pin = $PinNovo; nome = "troca-nova" },
                @{ pin = $PinNovo; nome = "troca-confirma" }
              )
            } else {
              @(@{ pin = $Pin; nome = "troca-atual" })
            }
            foreach ($etapa in $etapas) {
              $pngEtapa = ScreenshotRapido $handle "$Prefixo-$($etapa.nome)"
              $mapaEtapa = @{}
              foreach ($item in ([TelaCliente]::ReconheceTeclado($pngEtapa, $refsTexto, 190)).Split(";")) {
                $p = $item.Split(":")
                if ($p.Count -eq 4) { $mapaEtapa[$p[0]] = @([int]$p[1], [int]$p[2]) }
              }
              if ($mapaEtapa.Count -ne 10) {
                Escreve "$($etapa.nome): teclado nao reconhecido ($($mapaEtapa.Count) celulas)"
                break
              }
              foreach ($digito in $etapa.pin.ToCharArray()) {
                $alvo = $mapaEtapa["$digito"]
                if (-not $alvo) { Escreve "$($etapa.nome): digito '$digito' fora do teclado"; continue }
                Clique ($r.Left + $alvo[0]) ($r.Top + $alvo[1])
                Start-Sleep -Milliseconds 150
              }
              Escreve "$($etapa.nome): '$($etapa.pin)' digitado; clique em Alterar ($AlterarX,$AlterarY)"
              Clique ($r.Left + $AlterarX) ($r.Top + $AlterarY)
              Start-Sleep -Milliseconds 900
              Screenshot $handle "$Prefixo-$($etapa.nome)-pos"
            }
          } else {
            Escreve "clique em Confirmar (303,407) a $([int]((Get-Date) - $relogio).TotalMilliseconds) ms do inicio dos cliques"
            Clique ($r.Left + 303) ($r.Top + 407)
          }
        }
      } else {
        Escreve "digita PIN por scan code ($($Pin.Length) digitos)"
        [TelaCliente]::Texto($Pin) | Out-Null
        Start-Sleep -Milliseconds 400
        [TelaCliente]::Tecla(0x0D) | Out-Null
      }
      Start-Sleep -Milliseconds 1500
      Screenshot $handle "$Prefixo-pos-pin"
      if ($PainelEsperado -ne "") {
        $painel = (& powershell -NoProfile -ExecutionPolicy Bypass -File (Join-Path $raiz "tools/client/ocr-texto.ps1") `
            -Imagem "target/$Prefixo-pos-pin.png" -Texto $PainelEsperado -Escala 4 -Fatias $OcrFatias `
            -Minimo $OcrMinimo -Json "target/$Prefixo-painel.json" | Out-String).Trim()
        $codigoPainel = $LASTEXITCODE
        Escreve "ocr painel -> exit=$codigoPainel $painel"
        if ($codigoPainel -eq 0) { Escreve "veredito_painel=confere texto='$PainelEsperado'" }
        else { Escreve "veredito_painel=divergente texto='$PainelEsperado'" }
      }
      Start-Sleep -Seconds $EsperaSelecao
      Screenshot $handle "$Prefixo-selecao"
      if ($PersonagemX -gt 0 -and $PersonagemY -gt 0) {
        # Duplo clique no personagem abre o painel dele (medido em 2026-11-15: nome, classe,
        # nível, gold, atributos e o botão "Conectar"); **entrar** é o clique em Conectar —
        # sem ele o cliente não manda `_MSG_CharacterLogin` e a conexão fica parada na seleção.
        Escreve "duplo clique no personagem ($PersonagemX,$PersonagemY)"
        Clique ($r.Left + $PersonagemX) ($r.Top + $PersonagemY)
        Clique ($r.Left + $PersonagemX) ($r.Top + $PersonagemY)
        Start-Sleep -Milliseconds 1200
        Screenshot $handle "$Prefixo-painel"
        if ($PainelConectarX -gt 0 -and $PainelConectarY -gt 0) {
          Escreve "clique em Conectar do painel ($PainelConectarX,$PainelConectarY)"
          Clique ($r.Left + $PainelConectarX) ($r.Top + $PainelConectarY)
        }
        Start-Sleep -Seconds $EsperaMapa
        Screenshot $handle "$Prefixo-mapa"
        $est3 = Get-NetTCPConnection -RemotePort $Porta -State Established -ErrorAction SilentlyContinue
        Escreve "conexoes ESTABELECIDAS com ${Porta} no mapa: $(($est3 | Measure-Object).Count)"
      }
    }
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

foreach ($arquivo in @($outLog, $errLog, "target/$Prefixo-ocr-1.json", "target/$Prefixo-ocr-2.json", (Join-Path $raiz "target/captura-live80.out.log"), (Join-Path $raiz "client/WYD.log"))) {
  if (Test-Path $arquivo) { Escreve ("--- $arquivo`n" + (Get-Content $arquivo -Raw)) }
}
Escreve "=== fim ==="
