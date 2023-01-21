namespace StalewallLib;

public static class Queries {
    public static string GetQueriesFromcli(string[] args)
    {
        // TODO: actually make something better than just copy-paste the cli args
        string query = "";
        if (args.Length > 1)
        {
            query = $"?{args[1]}";
        }
        return query;
    }
}