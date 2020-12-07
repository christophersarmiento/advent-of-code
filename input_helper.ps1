$dir = "PATH TO AOC DIRECTORY";
$boilerplate = "PATH TO BOILERPLATE FILE";
$year = Get-Date -Format yyyy;
$day = (Get-Date -Format dd).TrimStart("0"," ");

Set-Location ($dir + $year);

$project_name = "day-" + (Get-Date -Format dd).ToString();

cargo new --bin $project_name;

Set-Location ((Get-Location).ToString() + "\day-" + (Get-Date -Format dd));

$input_path = (Get-Location).ToString() + "\src\input.txt";

$session = New-Object Microsoft.PowerShell.Commands.WebRequestSession;

$cookie = New-Object System.Net.Cookie;
$cookie.Name = "session";
$cookie.Value = "INSERT COOKIE HERE";
$cookie.Domain = "adventofcode.com";

$session.Cookies.Add($cookie);

$url = "https://adventofcode.com/" + $year + "/day/" + $day +"/input";

Invoke-WebRequest $url -OutFile $input_path -WebSession $session;

Copy-Item -Path $boilerplate -Destination ((Get-Location).ToString() + "\src\main.rs");

code ($dir + $year);

Start-Process firefox ("https://adventofcode.com/" + $year + "/day/" + $day)

Exit;