using System.Runtime.InteropServices;

namespace StalewallLib
{
    public static partial class Wallpaper
    {
        // Things to set wallpaper
        // Copied from https://github.com/EgeLosing/SunCore/blob/main/SunCore%20Ultralight/NT62x/Wallpaper.cs
        [LibraryImport("user32.dll", EntryPoint = "SystemParametersInfoA", SetLastError = true, StringMarshalling = StringMarshalling.Custom, StringMarshallingCustomType = typeof(System.Runtime.InteropServices.Marshalling.AnsiStringMarshaller))]
        public static partial int SystemParametersInfo(int uAction, int uParam, string lpvParam, int fuWinIni);

        private const int SPI_SETDESKWALLPAPER = 20;
        private const int SPIF_UPDATEINIFILE = 0x01;
        private const int SPIF_SENDWININICHANGE = 0x02;

        /// <summary>
        /// Change wallpaper on Windows
        /// </summary>
        /// <param name="path">Image path</param>
        public static void ChangeWallpaper(string path)
        {
            _ = SystemParametersInfo(SPI_SETDESKWALLPAPER, 0, path, SPIF_UPDATEINIFILE | SPIF_SENDWININICHANGE);
        }
    }
}