using System.Runtime.InteropServices;

namespace StalewallLib;

public static partial class Wallpaper
{
    // Things to set wallpaper
    // Copied from https://github.com/EgeLosing/SunCore/blob/main/SunCore%20Ultralight/NT62x/Wallpaper.cs
    [DllImport("user32.dll", CharSet = CharSet.Auto)]
    static extern int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);

    /// <summary>
    /// Change wallpaper on Windows
    /// </summary>
    /// <param name="path">Image path</param>
    public static void ChangeWallpaper(string path)
    {
        _ = SystemParametersInfo(20, 0, path, 0x01 | 0x02);
    }
}
