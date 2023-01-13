using System.Text.Json;

namespace StalewallLib
{
    public static class GetWall
    {
        /// <summary>
        /// Downloads a random image and returns path
        /// </summary>
        /// <returns>Image path</returns>
        public static async Task<string> GetWallpaper(string Args = "")
        {
            // Create path for image
            var path = Path.Combine(Path.GetTempPath(), "wall.jpg");
            // Get image url
            string GowallJSON = await Net.httpClient.GetStringAsync($"https://stalewall.vercel.app/api/{Args}");
            // Parse the JSON
            var wallx = JsonDocument.Parse(GowallJSON);
            var wally = wallx.RootElement.GetProperty("url").GetString();
            //Console.WriteLine(wally);
            // Download the image and return the path
            byte[] WallpaperBytes = await Net.httpClient.GetByteArrayAsync(wally);
            File.WriteAllBytes(path, WallpaperBytes);
            return path;
        }
    }
}