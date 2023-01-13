using StalewallLib;
using Windows.Foundation;
using Windows.Storage;
using Windows.System.UserProfile;

// Get commandline args
string[] Args = Environment.GetCommandLineArgs();
string query = "";
if (Args.Length > 1)
{
    query = $"?{Args[1]}";
}

// Ping 8.8.8.8
Net.Pong(2, 2, 2);

// Download Image
string imagePath = await GetWall.GetWallpaper(query);

// Get StorageFile from path
StorageFile image = await StorageFile.GetFileFromPathAsync(imagePath);

// Start setting lockscreen wallpaper
IAsyncAction lockscreen = LockScreen.SetImageFileAsync(image);

// Set desktop wallpaper
Wallpaper.ChangeWallpaper(imagePath);

// Await lockscreen wallpaper set
await lockscreen;