using StalewallLib;
using Windows.Foundation;
using Windows.Storage;
using Windows.System.UserProfile;

// Checks for internet connection
Net.CheckConnection();

// Get queries
string queries = Queries.GetQueriesFromcli(Environment.GetCommandLineArgs());
Console.WriteLine($"Queries: {queries}");

// Download Image
string imagePath = await GetWall.GetWallpaper(queries);
Console.WriteLine("Got wallpaper");

// Get StorageFile from path
StorageFile image = await StorageFile.GetFileFromPathAsync(imagePath);
Console.WriteLine("Loaded wallpaper for lockscreen");

// Start setting lockscreen wallpaper
IAsyncAction lockscreen = LockScreen.SetImageFileAsync(image);
Console.WriteLine("Setting wallpaper to lockscreen");

// Set desktop wallpaper
Wallpaper.ChangeWallpaper(imagePath);
Console.WriteLine("Desktop wallpaper set");

// Await lockscreen wallpaper set
await lockscreen;
Console.WriteLine("Lockscreen wallpaper set");