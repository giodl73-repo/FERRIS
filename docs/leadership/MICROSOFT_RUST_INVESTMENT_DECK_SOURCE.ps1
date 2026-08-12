param(
    [string]$OutputPath = (Join-Path $PSScriptRoot 'MICROSOFT_RUST_INVESTMENT_DECK.pptx'),
    [string]$RenderDirectory = (Join-Path $env:TEMP 'ferris-rust-investment-deck-rendered')
)

$ErrorActionPreference = 'Stop'

$ppt = New-Object -ComObject PowerPoint.Application
$ppt.Visible = -1
$presentation = $ppt.Presentations.Add()
$presentation.PageSetup.SlideWidth = 960
$presentation.PageSetup.SlideHeight = 540

$script:W = 960
$script:H = 540
$script:navy = 0x2D2110
$script:ink = 0x29231F
$script:cream = 0xF7F1E8
$script:paper = 0xFFFDFC
$script:rust = 0x2B4ACE
$script:orange = 0x315FD9
$script:teal = 0xC8B852
$script:muted = 0x776C64
$script:white = 0xFFFFFF
$script:line = 0xD8CEC5
$script:fontHead = 'Aptos Display'
$script:fontBody = 'Aptos'

function Add-Text {
    param($Slide, [string]$Text, [double]$X, [double]$Y, [double]$Width,
        [double]$Height, [double]$Size = 20, [int]$Color = $script:ink,
        [bool]$Bold = $false, [string]$Font = $script:fontBody,
        [int]$Align = 1)
    $shape = $Slide.Shapes.AddTextbox(1, $X, $Y, $Width, $Height)
    $shape.TextFrame2.MarginLeft = 0
    $shape.TextFrame2.MarginRight = 0
    $shape.TextFrame2.MarginTop = 0
    $shape.TextFrame2.MarginBottom = 0
    $shape.TextFrame2.WordWrap = -1
    $shape.TextFrame2.AutoSize = 0
    $range = $shape.TextFrame2.TextRange
    $range.Text = $Text
    $range.Font.Name = $Font
    $range.Font.Size = $Size
    $range.Font.Fill.ForeColor.RGB = $Color
    $range.Font.Bold = $(if ($Bold) { -1 } else { 0 })
    $range.ParagraphFormat.Alignment = $Align
    return $shape
}

function Add-Rect {
    param($Slide, [double]$X, [double]$Y, [double]$Width, [double]$Height,
        [int]$Fill, [int]$Radius = 1, [int]$LineColor = $Fill,
        [double]$Transparency = 0)
    $type = $(if ($Radius -eq 1) { 5 } else { 1 })
    $shape = $Slide.Shapes.AddShape($type, $X, $Y, $Width, $Height)
    $shape.Fill.ForeColor.RGB = $Fill
    $shape.Fill.Transparency = $Transparency
    $shape.Line.ForeColor.RGB = $LineColor
    $shape.Line.Transparency = $(if ($LineColor -eq $Fill) { 1 } else { 0 })
    return $shape
}

function Add-Circle {
    param($Slide, [double]$X, [double]$Y, [double]$Size, [int]$Fill)
    $shape = $Slide.Shapes.AddShape(9, $X, $Y, $Size, $Size)
    $shape.Fill.ForeColor.RGB = $Fill
    $shape.Line.Transparency = 1
    return $shape
}

function Add-Line {
    param($Slide, [double]$X1, [double]$Y1, [double]$X2, [double]$Y2,
        [int]$Color = $script:line, [double]$Weight = 1)
    $shape = $Slide.Shapes.AddLine($X1, $Y1, $X2, $Y2)
    $shape.Line.ForeColor.RGB = $Color
    $shape.Line.Weight = $Weight
    return $shape
}

function Add-Title {
    param($Slide, [string]$Title, [string]$Kicker = '')
    if ($Kicker) {
        Add-Text $Slide $Kicker 44 24 872 20 11 $script:rust $true | Out-Null
    }
    Add-Text $Slide $Title 44 50 872 48 30 $script:ink $true $script:fontHead | Out-Null
}

function Add-Footer {
    param($Slide, [string]$Source, [int]$Number)
    Add-Text $Slide $Source 44 494 810 15 7.5 $script:muted $false | Out-Null
    Add-Text $Slide ([string]$Number) 886 492 30 16 8 $script:muted $true $script:fontBody 3 | Out-Null
}

function Add-Pill {
    param($Slide, [string]$Text, [double]$X, [double]$Y, [double]$Width,
        [int]$Fill = $script:cream, [int]$Color = $script:ink)
    Add-Rect $Slide $X $Y $Width 26 $Fill | Out-Null
    Add-Text $Slide $Text ($X + 10) ($Y + 6) ($Width - 20) 14 10 $Color $true | Out-Null
}

function Add-Card {
    param($Slide, [string]$Heading, [string]$Body, [double]$X, [double]$Y,
        [double]$Width, [double]$Height, [int]$Accent = $script:rust)
    Add-Rect $Slide $X $Y $Width $Height $script:paper 1 $script:line | Out-Null
    Add-Rect $Slide $X $Y 8 $Height $Accent 0 | Out-Null
    Add-Text $Slide $Heading ($X + 22) ($Y + 16) ($Width - 38) 28 16 $script:ink $true | Out-Null
    Add-Text $Slide $Body ($X + 22) ($Y + 52) ($Width - 38) ($Height - 66) 11.5 $script:muted | Out-Null
}

# 1 - Title
$s = $presentation.Slides.Add(1, 12)
$s.Background.Fill.ForeColor.RGB = $script:navy
Add-Rect $s 0 0 960 540 $script:navy 0 | Out-Null
Add-Circle $s 690 -90 330 $script:rust | Out-Null
Add-Circle $s 770 40 250 $script:orange | Out-Null
Add-Circle $s 650 230 360 $script:teal | Out-Null
Add-Text $s 'MICROSOFT + RUST' 56 54 470 20 12 $script:orange $true | Out-Null
Add-Text $s 'Govern the conversion while the estate is still forming' 56 106 610 132 34 $script:white $true $script:fontHead | Out-Null
Add-Text $s 'A portfolio strategy for application blueprints, compiler-grounded AI, Windows and Azure differentiation, and fair upstream stewardship.' 58 264 570 84 17 0xD8E1E5 $false | Out-Null
Add-Pill $s 'Leadership discussion draft | 11 Aug 2026' 58 382 304 0x44362D $script:white
Add-Text $s 'FERRIS' 58 466 180 22 12 $script:teal $true | Out-Null

# 2 - Moment
$s = $presentation.Slides.Add(2, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'Rust has crossed from developer preference to platform strategy' 'THE MOMENT'
$stats = @(
    @{n='83%'; h='admired'; b='Stack Overflow 2024'},
    @{n='48.8%'; h='organizational use'; b='non-trivial use, Rust survey 2025'},
    @{n='315K'; h='published crates'; b='crates.io, 11 Aug 2026'},
    @{n='395B'; h='downloads'; b='cumulative crates.io downloads'}
)
for ($i=0; $i -lt 4; $i++) {
    $x = 44 + ($i * 222)
    Add-Rect $s $x 132 198 142 $script:paper 1 $script:line | Out-Null
    Add-Text $s $stats[$i].n ($x+18) 150 162 54 34 $script:rust $true $script:fontHead | Out-Null
    Add-Text $s $stats[$i].h ($x+18) 207 162 20 13 $script:ink $true | Out-Null
    Add-Text $s $stats[$i].b ($x+18) 235 162 28 10 $script:muted | Out-Null
}
Add-Rect $s 44 314 872 146 $script:navy | Out-Null
Add-Text $s 'The strategic shift' 66 337 220 26 16 $script:orange $true | Out-Null
Add-Text $s 'Rust is no longer a bet on language popularity. It is becoming a durable systems, cloud, security, and application-platform capability.' 66 374 780 58 22 $script:white $true $script:fontHead | Out-Null
Add-Footer $s 'Sources: Stack Overflow 2024; State of Rust 2025; crates.io public API (observed 2026-08-11).' 2

# 3 - Security economics
$s = $presentation.Slides.Add(3, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'The conversion case is strongest when it is incremental' 'SECURITY ECONOMICS'
Add-Text $s 'Android memory-safety vulnerabilities' 52 132 390 24 15 $script:ink $true | Out-Null
Add-Rect $s 52 178 370 54 $script:line | Out-Null
Add-Rect $s 52 178 281 54 $script:rust | Out-Null
Add-Text $s '76%' 66 188 80 34 24 $script:white $true $script:fontHead | Out-Null
Add-Text $s 'six years earlier' 340 196 74 20 10 $script:muted $true  $script:fontBody 3 | Out-Null
Add-Rect $s 52 262 370 54 $script:line | Out-Null
Add-Rect $s 52 262 89 54 $script:teal | Out-Null
Add-Text $s '24%' 66 272 80 34 24 $script:navy $true $script:fontHead | Out-Null
Add-Text $s '2024' 340 280 74 20 10 $script:muted $true $script:fontBody 3 | Out-Null
Add-Card $s 'Interoperability is the new rewrite' 'Prioritize memory-safe languages for new and actively changing code. Keep mature assets where rewrite economics are weak. Make boundaries safe, explicit, testable, and removable.' 486 136 408 180 $script:teal
Add-Card $s 'A second operational signal' 'Google reports the rollback rate of Rust changes at less than half the rate of C++ changes -- evidence that safety can improve developer operations, not only vulnerability counts.' 486 338 408 124 $script:orange
Add-Footer $s 'Source: Google Security Blog, "Eliminating Memory Safety Vulnerabilities at the Source," 25 Sep 2024.' 3

# 4 - Microsoft already moving
$s = $presentation.Slides.Add(4, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'Microsoft is already moving -- coordination is the missing layer' 'MICROSOFT SIGNALS'
$items = @(
    @('AZURE','Rust adopted in critical infrastructure; Microsoft expects adoption to expand substantially.'),
    @('SDK','Stable Azure SDK for Rust: core, identity, Key Vault, and Storage with SemVer guarantees.'),
    @('SYSTEMS','OpenVMM is a modular, cross-platform VMM written in Rust.'),
    @('FOUNDATION','Founding Platinum member since January 2021; an established channel for fair investment.')
)
for ($i=0; $i -lt 4; $i++) {
    $y = 125 + ($i * 86)
    Add-Circle $s 52 ($y+1) 44 $(if($i % 2 -eq 0){$script:rust}else{$script:teal}) | Out-Null
    Add-Text $s ([string]($i+1)) 65 ($y+10) 18 22 16 $(if($i % 2 -eq 0){$script:white}else{$script:navy}) $true $script:fontHead 2 | Out-Null
    Add-Text $s $items[$i][0] 116 $y 150 22 13 $script:rust $true | Out-Null
    Add-Text $s $items[$i][1] 116 ($y+27) 740 38 13 $script:ink | Out-Null
    if($i -lt 3){Add-Line $s 116 ($y+72) 896 ($y+72) $script:line 1 | Out-Null}
}
Add-Footer $s 'Sources: Microsoft Azure Security; Azure SDK Blog; github.com/microsoft/openvmm; Rust Foundation members.' 4

# 5 - Risk
$s = $presentation.Slides.Add(5, 12)
$s.Background.Fill.ForeColor.RGB = $script:navy
Add-Text $s 'THE RISK' 44 28 300 20 11 $script:orange $true | Out-Null
Add-Text $s 'Uncoordinated success becomes the next platform debt' 44 58 790 48 30 $script:white $true $script:fontHead | Out-Null
$risks = @(
    @('Crate divergence','Different stacks and providers for one capability'),
    @('Boundary drift','Ad hoc C++, C ABI, WIT, generated, and service contracts'),
    @('Support ambiguity','Versions, targets, owners, and renewal dates disappear'),
    @('AI without context','Generated native changes lack application scope'),
    @('Local-only CI','Workspaces pass while application impact remains unknown'),
    @('No exit path','Migration omits fallback, rollback, substitution, and removal')
)
for($i=0;$i -lt 6;$i++){
    $col=$i%3; $row=[math]::Floor($i/3); $x=44+($col*296); $y=138+($row*150)
    Add-Rect $s $x $y 270 124 0x44362D 1 | Out-Null
    Add-Text $s $risks[$i][0] ($x+18) ($y+18) 230 22 15 $script:orange $true | Out-Null
    Add-Text $s $risks[$i][1] ($x+18) ($y+52) 230 52 12 $script:white | Out-Null
}
Add-Text $s 'Cargo owns package truth. Microsoft still needs an application and portfolio layer.' 44 468 872 25 16 $script:teal $true | Out-Null
Add-Footer $s 'Ferris thesis: preserve owner authority; coordinate application intent, evidence, lifecycle, and affected work.' 5

# 6 - Blueprint
$s = $presentation.Slides.Add(6, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'Application blueprints turn Rust adoption into a supportable system' 'THE PROPOSED CONTROL PLANE'
$layers = @(
    @('1','Application intent','components | services | owners | policy'),
    @('2','Cargo truth','packages | targets | features | sources'),
    @('3','Contracts','Rust API | C ABI | CXX | WIT | wire IDL'),
    @('4','Supported profiles','crates | providers | platforms | renewal'),
    @('5','Evidence + lifecycle','security | validation | rollback | removal')
)
for($i=0;$i -lt 5;$i++){
    $x=72+($i*168); $y=154+([math]::Abs(2-$i)*18)
    Add-Rect $s $x $y 148 214 $(if($i -eq 2){$script:navy}else{$script:cream}) 1 $script:line | Out-Null
    Add-Circle $s ($x+50) ($y+18) 48 $(if($i -eq 2){$script:orange}else{$script:rust}) | Out-Null
    Add-Text $s $layers[$i][0] ($x+65) ($y+29) 18 20 14 $script:white $true $script:fontHead 2 | Out-Null
    Add-Text $s $layers[$i][1] ($x+14) ($y+82) 120 42 15 $(if($i -eq 2){$script:white}else{$script:ink}) $true $script:fontHead 2 | Out-Null
    Add-Text $s $layers[$i][2] ($x+16) ($y+135) 116 55 10.5 $(if($i -eq 2){0xD8E1E5}else{$script:muted}) $false $script:fontBody 2 | Out-Null
}
Add-Text $s 'One answer to: what is supported here, what changes now, what must be validated, and who may proceed?' 90 440 780 36 18 $script:rust $true $script:fontHead 2 | Out-Null
Add-Footer $s 'Blueprints are renewable evidence records -- not permanent approved-crate lists.' 6

# 7 - AI
$s = $presentation.Slides.Add(7, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s "Copilot's differentiated opportunity is governed native change" 'GITHUB + COPILOT'
$steps = @('Discover application','Plan affected work','Generate in scope','Compile + validate','Attach evidence','Human approval')
for($i=0;$i -lt 6;$i++){
    $x=48+($i*149)
    Add-Circle $s ($x+42) 157 56 $(if($i -eq 5){$script:teal}else{$script:rust}) | Out-Null
    Add-Text $s ([string]($i+1)) ($x+61) 173 18 20 15 $(if($i -eq 5){$script:navy}else{$script:white}) $true $script:fontHead 2 | Out-Null
    if($i -lt 5){Add-Line $s ($x+99) 185 ($x+147) 185 $script:orange 3 | Out-Null}
    Add-Text $s $steps[$i] $x 232 140 44 12 $script:ink $true $script:fontBody 2 | Out-Null
}
Add-Rect $s 100 320 760 106 $script:navy | Out-Null
Add-Text $s 'Not "Copilot writes Rust."' 132 342 330 28 18 $script:orange $true | Out-Null
Add-Text $s 'Copilot changes native systems with compiler-grounded, application-aware evidence.' 132 378 660 30 20 $script:white $true $script:fontHead | Out-Null
Add-Footer $s 'AI may propose scope. Deterministic policy controls narrowing; unknowns widen safely.' 7

# 8 - Two portfolios
$s = $presentation.Slides.Add(8, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'Invest in two portfolios -- and keep the boundary explicit' 'WHERE MICROSOFT PLAYS'
Add-Rect $s 44 128 412 316 $script:cream 1 $script:line | Out-Null
Add-Rect $s 504 128 412 316 0xEAF2F3 1 $script:line | Out-Null
Add-Text $s 'UPSTREAM PUBLIC GOOD' 70 154 350 24 16 $script:rust $true | Out-Null
Add-Text $s 'Earn trust through existing owners' 70 186 350 22 13 $script:muted | Out-Null
$left="rustc + Cargo performance`nWindows target, linker + debugger`nInterop patterns + conformance`nSupply chain + trusted publishing`nCritical crate stewardship`nAsync diagnostics + WIT`nPublic AI assurance fixtures"
Add-Text $s $left 70 230 340 176 14 $script:ink | Out-Null
Add-Text $s 'MICROSOFT DIFFERENTIATION' 530 154 350 24 16 $script:teal $true | Out-Null
Add-Text $s 'Build where Microsoft has asymmetric assets' 530 186 350 22 13 $script:muted | Out-Null
$right="GitHub Rust estate intelligence`nCopilot governed native change`nAzure build + provenance plane`nWindows enterprise Rust excellence`nRenewable application profiles`nCross-repository affected work`nEntra / Key Vault / policy connectors"
Add-Text $s $right 530 230 340 176 14 $script:ink | Out-Null
Add-Footer $s 'Rule: shared language infrastructure upstream; portable application governance plus Microsoft integrations in product.' 8

# 9 - Upstream
$s = $presentation.Slides.Add(9, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'Microsoft can earn durable community credit by funding the hard middle' 'UPSTREAM STEWARDSHIP'
$rows=@(
    @('Measure','Benchmarks, minimized regressions, cross-platform fixtures'),
    @('Align','Ask maintainers where evidence and implementation belong'),
    @('Contribute','Tests, patches, documentation, review and infrastructure'),
    @('Maintain','Fund releases, triage, compatibility, and succession'),
    @('Retire','Remove downstream work when upstream or consumer need changes')
)
for($i=0;$i -lt 5;$i++){
    $y=124+($i*70)
    Add-Rect $s 54 $y 150 50 $(if($i -eq 2){$script:rust}else{$script:navy}) | Out-Null
    Add-Text $s $rows[$i][0] 72 ($y+14) 114 20 14 $script:white $true | Out-Null
    Add-Text $s $rows[$i][1] 238 ($y+11) 640 34 13 $script:ink | Out-Null
}
Add-Text $s 'Success = accepted, maintained, broadly useful upstream outcomes -- not a Microsoft-owned fork.' 54 462 842 24 15 $script:rust $true $script:fontHead 2 | Out-Null
Add-Footer $s 'Initial targets: rustc-perf RDR benchmark, Cargo evidence, Windows tooling, generated-boundary provenance.' 9

# 10 - Pilot
$s = $presentation.Slides.Add(10, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'Prove the strategy on one real, removable application slice' 'FERRIS PILOT'
$criteria=@(
    '2+ Cargo workspaces',
    'C/C++ or native boundary',
    'Service or WIT contract',
    'Windows + Linux',
    'GitHub PR + CI',
    'Azure integration'
)
for($i=0;$i -lt 6;$i++){
    $col=$i%3; $row=[math]::Floor($i/3); $x=52+($col*296); $y=130+($row*88)
    Add-Rect $s $x $y 270 66 $script:cream 1 $script:line | Out-Null
    Add-Circle $s ($x+16) ($y+15) 34 $script:teal | Out-Null
    Add-Text $s 'OK' ($x+22) ($y+22) 22 16 9 $script:navy $true $script:fontHead 2 | Out-Null
    Add-Text $s $criteria[$i] ($x+62) ($y+20) 190 24 13 $script:ink $true | Out-Null
}
Add-Rect $s 52 334 862 116 $script:navy | Out-Null
Add-Text $s 'The pilot only passes if it can explain, renew, fall back, roll back, and remove.' 78 358 808 28 20 $script:white $true $script:fontHead 2 | Out-Null
Add-Text $s 'And it must create at least one useful upstream contribution packet.' 78 402 808 22 14 $script:orange $true $script:fontBody 2 | Out-Null
Add-Footer $s 'Ferris remains read-first and Cargo-native; execution authority expands only through separate governed gates.' 10

# 11 - Roadmap
$s = $presentation.Slides.Add(11, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'A staged investment keeps ambition high and risk bounded' 'ROADMAP'
$phases=@(
    @('0-90 DAYS','Sponsor + discover',"Estate census`nDesign partners`nBaseline + upstream liaison"),
    @('3-6 MONTHS','Blueprint proof',"One application`nGitHub evidence flow`nWindows/Linux + removal"),
    @('6-12 MONTHS','Platform pilot',"Multiple orgs`nRead-only MCP`nCopilot + Azure connectors"),
    @('12-24 MONTHS','Selective product',"Supported capabilities`nRenewable profiles`nSLAs + outcomes")
)
for($i=0;$i -lt 4;$i++){
    $x=44+($i*222)
    Add-Rect $s $x 142 198 280 $(if($i -eq 0){$script:navy}else{$script:paper}) 1 $script:line | Out-Null
    Add-Text $s $phases[$i][0] ($x+16) 162 166 18 10 $(if($i -eq 0){$script:orange}else{$script:rust}) $true | Out-Null
    Add-Text $s $phases[$i][1] ($x+16) 202 166 48 18 $(if($i -eq 0){$script:white}else{$script:ink}) $true $script:fontHead | Out-Null
    Add-Text $s $phases[$i][2] ($x+16) 276 166 104 12 $(if($i -eq 0){0xD8E1E5}else{$script:muted}) | Out-Null
}
Add-Line $s 88 454 872 454 $script:rust 4 | Out-Null
foreach($x in @(88,310,532,754,872)){Add-Circle $s ($x-7) 447 14 $script:orange | Out-Null}
Add-Footer $s 'Stage gates: owner demand, measurable value, cross-platform proof, policy safety, adoption, rollback, and removal.' 11

# 12 - Ask
$s = $presentation.Slides.Add(12, 12)
$s.Background.Fill.ForeColor.RGB = $script:navy
Add-Text $s 'THE LEADERSHIP ASK' 48 32 350 20 11 $script:orange $true | Out-Null
Add-Text $s 'Sponsor the application strategy -- not another isolated Rust tool' 48 66 820 64 31 $script:white $true $script:fontHead | Out-Null
$asks=@(
    'Six-month cross-company proof',
    'One systems application + one GitHub/Copilot workflow',
    'Privacy-safe Rust estate census',
    'Dedicated upstream liaison + maintainer funding',
    'Windows/Linux, renewal, rollback, and removal gates'
)
for($i=0;$i -lt 5;$i++){
    $y=164+($i*58)
    Add-Rect $s 96 ($y-5) 760 42 0x44362D 1 | Out-Null
    Add-Circle $s 54 ($y+1) 32 $(if($i -eq 4){$script:teal}else{$script:orange}) | Out-Null
    Add-Text $s ([string]($i+1)) 65 ($y+9) 12 16 11 $script:navy $true $script:fontHead 2 | Out-Null
    Add-Text $s $asks[$i] 116 ($y+5) 714 28 15 $script:white $true | Out-Null
}
Add-Rect $s 48 456 864 40 0x44362D | Out-Null
Add-Text $s 'Grab the moment: align the workspaces, crates, contracts, and applications before fragmentation hardens.' 70 467 820 18 13 $script:teal $true $script:fontBody 2 | Out-Null

$presentation.SaveAs($OutputPath, 24)

if (Test-Path $RenderDirectory) {
    Get-ChildItem $RenderDirectory -File -Filter '*.png' | Remove-Item
} else {
    New-Item -ItemType Directory -Path $RenderDirectory | Out-Null
}

foreach ($slide in $presentation.Slides) {
    $name = 'slide-{0:D2}.png' -f $slide.SlideIndex
    $slide.Export((Join-Path $RenderDirectory $name), 'PNG', 1600, 900)
}

$presentation.Close()
$ppt.Quit()
[System.Runtime.InteropServices.Marshal]::ReleaseComObject($presentation) | Out-Null
[System.Runtime.InteropServices.Marshal]::ReleaseComObject($ppt) | Out-Null

Write-Output $OutputPath
Write-Output $RenderDirectory
