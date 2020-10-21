package sgf;

import java.awt.event.KeyEvent;
import java.awt.Robot;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.stream.Collectors;

public class KeyCommand
{
    /** Key Code of the primary key to press */
    public final int keyCode;
    /** List of modifier to hold down while pressing the primary key */
    public final List<Modifier> modifiers;

    /**
     * Construct a KeyCommand for a single key.
     *
     * @param keyCode Key code of the key to press (see {@link java.awt.event.KeyEvent})
     */
    public KeyCommand(final int keyCode) {
        this(keyCode, new ArrayList<>());
    }

    /**
     * Construct a KeyCommand for a single key with modifiers.
     *
     * @param keyCode Key code of the key to press (see {@link java.awt.event.KeyEvent})
     * @param modifiers
     */
    public KeyCommand(final int keyCode, final Modifier... modifiers) {
        this(keyCode, Arrays.asList(modifiers));
    }

    /**
     * Construct a KeyCommand for a single key with modifiers.
     *
     * @param keyCode Key code of the key to press (see {@link java.awt.event.KeyEvent})
     * @param modifiers
     */
    public KeyCommand(final int keyCode, final List<Modifier> modifiers) {
        this.keyCode = keyCode;
        this.modifiers = modifiers;
    }

    /**
     * Parse a KeyCommand from a string of the format:
     * <pre>
     * &lt;key-command&gt; ::= (&lt;modifier&gt; "+")* &lt;character&gt;
     * &lt;modifier&gt;    ::= "shift" | "ctrl" | "alt" | "meta"
     * &lt;character&gt;   ::= any keyboard character, probably
     * </pre>
     * TODO: support non-single-character keys.
     *
     * @param command
     * @return The parsed KeyCommand
     * @throws KeyCommandException
     */
    public static KeyCommand parse(String command) throws KeyCommandException
    {
        final String[] components = command.split("\\+");
        final String keyString = components[components.length - 1];
        if(keyString.length() != 1) {
            throw new KeyCommandException(command, "Invalid key length [" + keyString.length() + "] for key '" + keyString + "'");
        }
        final int keyCode = KeyEvent.getExtendedKeyCodeForChar(keyString.charAt(0));

        final List<Modifier> modifiers = Arrays.stream(components).limit(components.length - 1)
            .map(Util.errorWrapper(Modifier::parse))
            .collect(Collectors.toList());

        return new KeyCommand(keyCode, modifiers);
    }

    /**
     * Performs the KeyCommand using a given Robot.
     *
     * @param robot
     */
    public void execute(final Robot robot)
    {
        for(final Modifier mod : this.modifiers) {
            robot.keyPress(mod.keyCode);
        }
        robot.keyPress(this.keyCode);
        robot.keyRelease(this.keyCode);
        for(final Modifier mod : this.modifiers) {
            robot.keyRelease(mod.keyCode);
        }
    }



    /**
     * Modifier keys that can be held while performing a key command.
     */
    public static enum Modifier
    {
        SHIFT(KeyEvent.VK_SHIFT),
        CTRL (KeyEvent.VK_CONTROL),
        ALT  (KeyEvent.VK_ALT),
        META (KeyEvent.VK_META);

        protected int keyCode;
        private Modifier(final int keyCode)
        {
            this.keyCode = keyCode;
        }

        public static Modifier parse(String modifierString) throws KeyCommandException
        {
            for (Modifier modifier : Modifier.values()) {
                if(modifierString.equals(modifier.name().toLowerCase())) {
                    return modifier;
                }
            }
            throw new KeyCommandException("Invalid key modifier '" + modifierString + "'");
        }
    }
}
