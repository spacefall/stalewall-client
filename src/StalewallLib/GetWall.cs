using System.Text.Json;

namespace StalewallLib;

public static class GetWall
{
    /// <summary>
    /// Downloads a random image and returns path
    /// </summary>
    /// <returns>Image path</returns>
    public static async Task<string> GetWallpaper(string argz = "")
    {
        // Create path for image
        var path = Path.Combine(Path.GetTempPath(), "wall.jpg");
        // Get image url
        string gowallJson = await Net.PreInitHttpClient.GetStringAsync($"https://stalewall.vercel.app/api/{argz}");
        // Parse the JSON
        var wallx = JsonDocument.Parse(gowallJson);
        var wally = wallx.RootElement.GetProperty("url").GetString();
        //Console.WriteLine(wally);
        // Download the image and return the path
        byte[] wallpaperBytes = await Net.PreInitHttpClient.GetByteArrayAsync(wally);
        await File.WriteAllBytesAsync(path, wallpaperBytes);
        return path;
    }
}
