Wrap Unicode text with ANSI color codes

```rust
use termwrap::termwrap;

assert_eq!(
    termwrap(
        "\
Etiam labefactant saepe rem \u{1b}[33mpublicam\u{1b}[0m. Ex cupiditatibus \
odia, discidia, discordiae, seditiones, bella nascuntur, nec eae se foris \
\u{1b}[38;5;12msolum iactant\u{1b}[0m nec tantum in alios caeco impetu \
incurrunt, sed intus etiam in animis nostris inesse notionem, ut alterum esse \
appetendum, alterum aspernandum sentiamus. Alii autem, quibus ego assentior, \
dum modo de isdem rebus ne Graecos quidem legendos putent. Res vero bonas \
verbis electis graviter ornateque dictas quis non legat? Nisi qui se dicant \
in Graecis legendis operam malle consumere.
\
        ",
        50,
        "\\",
    ),
    "\
Etiam labefactant saepe rem \u{1b}[33mpublicam\u{1b}[0m. Ex cupidita\\
tibus odia, discidia, discordiae, seditiones, bel\\
la nascuntur, nec eae se foris \u{1b}[38;5;12msolum iactant\u{1b}[0m nec \\
tantum in alios caeco impetu incurrunt, sed intus\\
 etiam in animis nostris inesse notionem, ut alte\\
rum esse appetendum, alterum aspernandum sentiamu\\
s. Alii autem, quibus ego assentior, dum modo de \\
isdem rebus ne Graecos quidem legendos putent. Re\\
s vero bonas verbis electis graviter ornateque di\\
ctas quis non legat? Nisi qui se dicant in Graeci\\
s legendis operam malle consumere.
\
    ",
);
```
