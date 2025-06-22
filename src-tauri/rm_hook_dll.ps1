$process = Get-Process -Name "clash-verge" #process name with no extension needed
$module = $process.Modules | Where-Object {$_.ModuleName -like "interceptor_hotkey.dll"}
Write-Host $module
exit
if ($module){
$addr = $module.BaseAddress
$kernel32 = Add-Type -Name "Kernel32" -Namespace "Win32" -PassThru -MemberDefinition
@"
    [DllImport("kernel32.dll")] public static extern bool FreeLibrary(IntPtr hModule);
"@
$kernel32::FreeLibrary($addr)
}
