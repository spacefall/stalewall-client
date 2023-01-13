using System.Net;
using System.Net.NetworkInformation;

namespace StalewallLib
{
    public static class Net
    {
        /// <summary>
        /// Preinitialized HttpClient()
        /// </summary>
        public static readonly HttpClient httpClient = new();

        /// <summary>
        /// Checks if the pc is connected to the internet
        /// </summary>
        /// <param name="timeout">Time in seconds to wait before timing out</param>
        /// <param name="maxRetries">Times to retry ping before throwing exception</param>
        /// <param name="sleepBetweenRetries">Time to sleep in between failures</param>
        /// <returns>
        /// Returns boolean, true if connected, false if not
        /// </returns>
        /// <exception cref="ArgumentOutOfRangeException"></exception>
        /// <exception cref="WebException"></exception>
        public static bool Pong(int timeout, int maxRetries, int sleepBetweenRetries)
        {
            if (maxRetries < 0) { throw new ArgumentOutOfRangeException(nameof(maxRetries), "maxRetries cannot be 0 or less"); }

            Ping pinger = new();
            for (int retry = 1; retry < maxRetries; retry++)
            {
                // Pings 1.1.1.1 for pings times
                PingReply result = pinger.Send("1.1.1.1", timeout);

                // If status =/= "Success" then retry after (sleepBetweenRetries * 1000) seconds
                if (result.Status.ToString() != "Success")
                {
                    Console.WriteLine($"Couldn't ping, retrying in {sleepBetweenRetries} seconds. Retires left: {maxRetries - retry}\n" +
                                        $"Error code: {result.Status}");
                    Thread.Sleep(sleepBetweenRetries * 1000);
                }
                // Else just return true
                else
                {
                    return true;
                }
            }
            throw new WebException($"Ping failed {maxRetries} times.");
        }

        /// <summary>
        /// Downloads an image from url and saves it to path
        /// </summary>
        /// <param name="url">Image source</param>
        /// <param name="path">Path to destination</param>
        public static async Task DownloadImage(string url, string path)
        {
            byte[] fileBytes = await httpClient.GetByteArrayAsync(url);
            File.WriteAllBytes(path, fileBytes);
        }
    }
}