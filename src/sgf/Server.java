package sgf;

import java.awt.AWTException;
import java.awt.Robot;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.PrintWriter;
import java.net.ServerSocket;
import java.net.Socket;

public class Server
{
    /**
     * Listen for a connection. Currently only allows ONE (1) connection at a time.
     *
     * @param port
     * @throws IOException
     */
    public static void listen(short port) throws IOException
    {
        final ServerSocket serverSocket = new ServerSocket(port);
        System.out.printf("Macro Server listening on port %d...\n", port);

        /** Connection Count, used as a unique connection ID */
        int connCount = 0;
        boolean hasQuit = false;
        // Listens until a client sends the `quit` command
        while(!hasQuit)
        {
            final Socket clientSocket = serverSocket.accept();
            hasQuit = Server.handleClient(clientSocket, connCount);
            connCount++;
        }

        System.out.println("Closing Macro Server");
        serverSocket.close();
    }

    /**
     * Handles a client session.
     *
     * @param socket Client socket to read from and write to
     * @param cid Connection ID
     * @return True is the server should quit, false otherwise
     * @throws IOException
     */
    private static boolean handleClient(final Socket socket, final int cid) throws IOException
    {
        boolean hasQuit = false;
        System.out.printf("New connection [#%d]: %s\n", cid, socket.getInetAddress().toString());
        final BufferedReader inStream = new BufferedReader(new InputStreamReader(socket.getInputStream()));
        final PrintWriter outStream = new PrintWriter(socket.getOutputStream());

        String line;
        CLIENT_LOOP:
        while((line = inStream.readLine()) != null)
        {
            // Use whitespace to find the command versus the rest of the string
            final int splitIndex = line.contains(" ") ? line.indexOf(" ") : line.length();
            final String command = line.substring(0, splitIndex).trim();
            final String args    = line.substring(splitIndex).trim();

            if(args.length() > 0) {
                System.out.printf("    [#%d] [%s]: %s\n", cid, command, args);
            } else {
                System.out.printf("    [#%d] [%s]\n", cid, command);
            }

            switch(command)
            {
                case "quit": hasQuit = true;
                case "exit": break CLIENT_LOOP;
                case "key":
                    try {
                        Server.processMacro(cid, args);
                    } catch(AWTException awte) {
                        System.err.println("Error processing macro:");
                        awte.printStackTrace();
                        outStream.println("Error processing macro");
                    } catch(KeyCommandException kce) {
                        System.err.println("Error parsing key command:");
                        kce.printStackTrace();
                        outStream.println(kce.getMessage());
                    }
                    break;
                default:
                    System.out.println("        unknown command");
            }
        }
        System.out.printf("Disconnecting [#%d]...\n", cid);
        socket.close();
        return hasQuit;
    }

    /**
     * Parses and executes a macro.
     * @see KeyCommand#parse(String)
     *
     * @param cid Connection ID
     * @param command Key command string to parse and execute
     * @throws AWTException
     * @throws KeyCommandException
     */
    private static void processMacro(final int cid, final String command) throws AWTException, KeyCommandException
    {
        final KeyCommand keyCommand = KeyCommand.parse(command);
        // Could make robot an instance of Server to persist it, but not necessary right now.
        final Robot robot = new Robot();
        keyCommand.execute(robot);
    }
}
