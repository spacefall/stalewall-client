using System.Net.NetworkInformation;

namespace StalewallLib;

public static class Net
{
    /// <summary>
    /// Preinitialized HttpClient()
    /// </summary>
    public static readonly HttpClient PreInitHttpClient = new();

    public static void CheckConnection()
    {
        // Checking for internet connection
        bool isNetworkConnected = NetworkInterface.GetIsNetworkAvailable();
        if (isNetworkConnected == false)
        {
            Console.WriteLine("You're not connected to the interwebs. Quitting...");
            Environment.Exit(1);
        }
        Console.WriteLine("You're connected to the interwebs.");
    }
}
