package sgf;

import java.util.Optional;

public class KeyCommandException extends Exception
{
    private static final long serialVersionUID = 1L;

    public final Optional<String> command;



    public KeyCommandException(final String message)
    {
        this(Optional.empty(), message);
    }
    public KeyCommandException(final String command, final String message)
    {
        this(Optional.of(command), message);
    }
    private KeyCommandException(final Optional<String> command, final String message)
    {
        super(message);
        this.command = command;
    }
}
