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
Add-Pill $s 'Leadership discussion draft | 21 Sep 2026' 58 382 304 0x44362D $script:white
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
Add-Footer $s 'Claims P-01 to P-04: MICROSOFT_RUST_CLAIM_LEDGER.md.' 2

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
Add-Footer $s 'Claims P-05 and P-06: MICROSOFT_RUST_CLAIM_LEDGER.md.' 3

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
Add-Footer $s 'Claims M-01 to M-04: MICROSOFT_RUST_CLAIM_LEDGER.md.' 4

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
Add-Footer $s 'Ferris demonstrates this layer without replacing Cargo, required CI, or owner authority.' 5

# 6 - Current proof
$s = $presentation.Slides.Add(6, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'Ferris now executes approved owner work and records evidence' 'CURRENT PRODUCT PROOF'
$proof = @(
    @('PLAN','Cargo-native planning preserves owner declarations and widens unknown inputs safely.'),
    @('EXECUTE','Approved Action Plans launch only exact repository-owned commands.'),
    @('EVIDENCE','Receipts preserve direct and transitive blockers across verification and replay.'),
    @('PREFLIGHT','Workspace and application readiness identify missing requirements before owner work.')
)
for($i=0;$i -lt 4;$i++){
    $col=$i%2; $row=[math]::Floor($i/2); $x=52+($col*438); $y=132+($row*138)
    Add-Card $s $proof[$i][0] $proof[$i][1] $x $y 410 112 $(if($i -eq 2){$script:teal}else{$script:rust})
}
Add-Rect $s 52 422 848 48 $script:navy | Out-Null
Add-Text $s 'Owner commands remain authoritative. Ferris does not remove or replace required CI gates.' 76 435 800 22 16 $script:white $true $script:fontHead 2 | Out-Null
Add-Footer $s 'Claims F-03 and F-04: MICROSOFT_RUST_CLAIM_LEDGER.md. No production support claim.' 6

# 7 - AI
$s = $presentation.Slides.Add(7, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'Target Copilot workflow for governed native change' 'FUTURE WORKFLOW, NOT IMPLEMENTED'
$steps = @('Discover application','Plan affected work','Generate in scope','Compile + validate','Attach evidence','Human approval')
for($i=0;$i -lt 6;$i++){
    $x=48+($i*149)
    Add-Circle $s ($x+42) 157 56 $(if($i -eq 5){$script:teal}else{$script:rust}) | Out-Null
    Add-Text $s ([string]($i+1)) ($x+61) 173 18 20 15 $(if($i -eq 5){$script:navy}else{$script:white}) $true $script:fontHead 2 | Out-Null
    if($i -lt 5){Add-Line $s ($x+99) 185 ($x+147) 185 $script:orange 3 | Out-Null}
    Add-Text $s $steps[$i] $x 232 140 44 12 $script:ink $true $script:fontBody 2 | Out-Null
}
Add-Rect $s 100 320 760 106 $script:navy | Out-Null
Add-Text $s 'The opportunity is governed native change.' 132 342 500 28 18 $script:orange $true | Out-Null
Add-Text $s 'Record revision, model action, commands, results, and human approval.' 132 378 660 30 19 $script:white $true $script:fontHead | Out-Null
Add-Footer $s 'Target only. Ferris has no GitHub, Azure, Copilot, or MCP integration today.' 7

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
Add-Footer $s 'Candidate areas only. Select after maintainer alignment, a named consumer, and a build-versus-contribute review.' 9

# 10 - Proof and next pilot
$s = $presentation.Slides.Add(10, 12)
$s.Background.Fill.ForeColor.RGB = $script:paper
Add-Title $s 'One enterprise gate passed; measured value remains the blocker' 'CURRENT GATE STATUS'
$criteria=@(
    @('PASSED','Gate 1: onboarding + removal'),
    @('PROOF','Windows/Linux owner execution'),
    @('PROOF','Receipts + clean post-removal runs'),
    @('BLOCKED','Gate 2: measured value'),
    @('WAITING','Gate 3: advisory CI shadow'),
    @('WAITING','Gate 4: support boundary')
)
for($i=0;$i -lt 6;$i++){
    $col=$i%3; $row=[math]::Floor($i/3); $x=52+($col*296); $y=130+($row*88)
    Add-Rect $s $x $y 270 66 $script:cream 1 $script:line | Out-Null
    $badge = $(if($criteria[$i][0] -in @('PASSED','PROOF')){$script:teal}elseif($criteria[$i][0] -eq 'BLOCKED'){$script:orange}else{$script:rust})
    Add-Circle $s ($x+16) ($y+15) 34 $badge | Out-Null
    Add-Text $s $(if($criteria[$i][0] -in @('PASSED','PROOF')){'OK'}elseif($criteria[$i][0] -eq 'BLOCKED'){'NO'}else{'--'}) ($x+20) ($y+22) 26 16 8 $script:navy $true $script:fontHead 2 | Out-Null
    Add-Text $s $criteria[$i][0] ($x+62) ($y+10) 190 16 9 $script:rust $true | Out-Null
    Add-Text $s $criteria[$i][1] ($x+62) ($y+29) 190 24 12.5 $script:ink $true | Out-Null
}
Add-Rect $s 52 334 862 116 $script:navy | Out-Null
Add-Text $s '37 of 40 historical revisions widened to full validation.' 78 352 808 28 19 $script:white $true $script:fontHead 2 | Out-Null
Add-Text $s 'The only admissible narrowed comparison was 7.7% slower. Later cohorts produced no admissible timing.' 78 393 808 38 14 $script:orange $true $script:fontBody 2 | Out-Null
Add-Footer $s 'Claims F-01 and F-02: MICROSOFT_RUST_CLAIM_LEDGER.md. No savings, CI replacement, or support claim.' 10

# 11 - Roadmap
$s = $presentation.Slides.Add(11, 12)
$s.Background.Fill.ForeColor.RGB = $script:cream
Add-Title $s 'Evidence gates control the roadmap' 'GATED ROADMAP'
$phases=@(
    @('PASSED','Foundation proof',"Owner execution + receipts`nWindows/Linux onboarding`nComplete removal proof"),
    @('PROPOSED','One final value cohort',"Frozen owner preflight`nOne attempt maximum`nStop if invalid, divergent,`nor nonpositive"),
    @('BLOCKED','Advisory CI shadow',"Owner-approved scope`nRequired checks retained`nDivergence measured"),
    @('BLOCKED','Support + interop',"Platforms + ABI`nOwnership + allocation`nPanic/error + threading`nDebugging + deployment`nRollback + audit + support")
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
Add-Footer $s 'A later gate opens only after the preceding evidence passes owner review.' 11

# 12 - Ask
$s = $presentation.Slides.Add(12, 12)
$s.FollowMasterBackground = 0
$s.Background.Fill.Solid()
$s.Background.Fill.ForeColor.RGB = $script:navy
Add-Text $s 'THE LEADERSHIP ASK' 48 32 350 20 11 $script:orange $true | Out-Null
Add-Text $s 'Fund the evidence needed for a platform decision' 48 66 820 64 31 $script:white $true $script:fontHead | Out-Null
$asks=@(
    'Name one accountable sponsor and one application owner',
    'Authorize one preflight-qualified value cohort, one attempt maximum',
    'Authorize an advisory CI shadow only after Gate 2 passes',
    'Complete the Microsoft Rust estate and economic baseline',
    'Define support, interop, rollback, audit, and an explicit exit path'
)
for($i=0;$i -lt 5;$i++){
    $y=164+($i*58)
    Add-Rect $s 96 ($y-5) 760 42 0x44362D 1 | Out-Null
    Add-Circle $s 54 ($y+1) 32 $(if($i -eq 4){$script:teal}else{$script:orange}) | Out-Null
    Add-Text $s ([string]($i+1)) 65 ($y+9) 12 16 11 $script:navy $true $script:fontHead 2 | Out-Null
    Add-Text $s $asks[$i] 116 ($y+5) 714 28 15 $script:white $true | Out-Null
}
Add-Rect $s 48 456 864 40 0x44362D | Out-Null
Add-Text $s 'Stop after an invalid, divergent, or nonpositive cohort. Platform funding remains gated.' 70 467 820 18 13 $script:teal $true $script:fontBody 2 | Out-Null

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
