package sgf;

import java.io.IOException;

public class App
{
    public static final short PORT = 8073;

    public static void main(String[] args)
    {
        try {
            Server.listen(PORT);
        } catch(IOException ioe) {
            System.err.println("Encountered error while listening for macros:");
            ioe.printStackTrace();
        }
    }
}
