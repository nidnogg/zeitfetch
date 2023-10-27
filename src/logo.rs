pub fn get_logo_by_distro(sys_name: &str) -> String {
    match sys_name {
        "deb" => String::from(
            "
\x1b[1m       _,.ss$$$$$gg.
\x1b[1m    ,g$$$$$$$$$$$$$$$P.
\x1b[1m  ,g$$P`'     ```Y$$.`.
\x1b[1m ,$$P''              `$$$.
\x1b[1m',$$P       ,ggs.     `$$b:
\x1b[1m`d$$'     ,$P`'   \x1b[91;1m.\x1b[0m    $$$
\x1b[1m$$P'      d$'     \x1b[91;1m,\x1b[0m    $$P
\x1b[1m$$:      $$.   \x1b[91;1m-\x1b[0m    ,d$$`
\x1b[1m$$;      Y$b._   _,d$P'
\x1b[1mY$$.    \x1b[91;1m`.\x1b[0m``Y$$$$P`'
\x1b[1m`$$b      \x1b[91;1m`\"-.__\x1b[0m
\x1b[1m `Y$$
\x1b[1m  `Y$$.
\x1b[1m    `$$b.
\x1b[1m       `Y$$b.
\x1b[1m          ``Y$b._
\x1b[1m              ```\x1b[0m
            ",
        ),
        "ubuntu" => String::from(
            "
\x1b[1m                        \x1b[91;1m./+o+-\x1b[0m
\x1b[1m                yyyyy- \x1b[91;1m-yyyyyy+\x1b[0m
\x1b[1m             ://+//////\x1b[91;1m-yyyyyyo\x1b[0m
\x1b[1m         \x1b[93;1m.++\x1b[0m .:/++++++/-.+sss/`\x1b[0m
\x1b[1m       \x1b[93;1m.:++o:\x1b[0m  /++++++++/:--:/-
\x1b[1m      \x1b[93;1mo:+o+:++.\x1b[0m`..```.-/oo+++++/
\x1b[1m     \x1b[93;1m.:+o:+o/.\x1b[0m          `+sssoo+/
\x1b[1m.++/+:\x1b[93;1m+oo+o:`\x1b[0m             /sssooo.
\x1b[1m/+++//+:\x1b[93;1m`oo+o\x1b[0m               /::--:.
\x1b[1m\\+/+o+++\x1b[93;1m`o++o\x1b[0m               \x1b[91;1m++////.\x1b[0m
\x1b[1m .++.o+\x1b[93;1m++oo+:`\x1b[0m             \x1b[91;1m/dddhhh.\x1b[0m
\x1b[1m      \x1b[93;1m.+.o+oo:.\x1b[0m          \x1b[91;1m`oddhhhh+\x1b[0m
\x1b[1m       \x1b[93;1m\\+.++o+o`\x1b[0m\x1b[91;1m`-````.:ohdhhhhh+\x1b[0m
\x1b[1m        \x1b[93;1m`:o+++\x1b[0m \x1b[91;1m`ohhhhhhhhyo++os:\x1b[0m
\x1b[1m          \x1b[93;1m.o:\x1b[0m`\x1b[91;1m.syhhhhhhh/\x1b[0m\x1b[93;1m.oo++o`\x1b[0m
\x1b[1m              \x1b[91;1m/osyyyyyyo\x1b[93;1m++ooo+++/\x1b[0m
\x1b[1m                  \x1b[91;1m`````\x1b[0m \x1b[93;1m+oo+++o\\:\x1b[0m
\x1b[1m                         \x1b[93;1m`oo++.\x1b[0m
                ",
        ),
        "fedora" => String::from(
            "
\x1b[94;1m             .',;::::;,'.
\x1b[94;1m         .';:cccccccccccc:;,.
\x1b[94;1m      .;cccccccccccccccccccccc;.
\x1b[94;1m    .:cccccccccccccccccccccccccc:.
\x1b[94;1m  .;ccccccccccccc;\x1b[0m.:dddl:.\x1b[94;1m;ccccccc;.
\x1b[94;1m .:ccccccccccccc;\x1b[0mOWMKOOXMWd\x1b[94;1m;ccccccc:.
\x1b[94;1m.:ccccccccccccc;\x1b[0mKMMc\x1b[94;1m;cc;\x1b[0mxMMc\x1b[94;1m:ccccccc:.
\x1b[94;1m,cccccccccccccc;\x1b[0mMMM.\x1b[94;1m;cc;\x1b[0m;WW:\x1b[94;1m:cccccccc,
\x1b[94;1m:cccccccccccccc;\x1b[0mMMM.\x1b[94;1m;cccccccccccccccc:
\x1b[94;1m:ccccccc;\x1b[0moxOOOo\x1b[94;1m;\x1b[0mMMM0OOk.\x1b[94;1m;cccccccccccc:
\x1b[94;1mcccccc:\x1b[0m0MMKxdd:\x1b[94;1m;\x1b[0mMMMkddc.\x1b[94;1m;cccccccccccc;
\x1b[94;1mccccc:\x1b[0mXM0'\x1b[94;1m;cccc;\x1b[0mMMM.\x1b[94;1m;cccccccccccccccc'
\x1b[94;1mccccc;\x1b[0mMMo\x1b[94;1m;ccccc;\x1b[0mMMW.\x1b[94;1m;ccccccccccccccc;
\x1b[94;1mccccc;\x1b[0m0MNc.\x1b[94;1mccc\x1b[0m.xMMd\x1b[94;1m:ccccccccccccccc;
\x1b[94;1mcccccc;\x1b[0mdNMWXXXWM0:\x1b[94;1m:cccccccccccccc:,
\x1b[94;1mcccccccc;\x1b[0m.:odl:.\x1b[94;1m;cccccccccccccc:,.
\x1b[94;1m:cccccccccccccccccccccccccccc:'.
\x1b[94;1m.:cccccccccccccccccccccc:;,..
\x1b[94;1m  '::cccccccccccccc::;,.\x1b[0m",
        ),
        "mac" => String::from(
            "
\x1b[92;1m                   'c.
\x1b[92;1m                ,xNMM.
\x1b[92;1m              .OMMMMo
\x1b[92;1m              OMMM0,
\x1b[92;1m    .;loddo:' loolloddol;.
\x1b[92;1m  cKMMMMMMMMMMNWMMMMMMMMMM0:
\x1b[93;1m.KMMMMMMMMMMMMMMMMMMMMMMMWd.
\x1b[93;1mXMMMMMMMMMMMMMMMMMMMMMMMX.
\x1b[91;1m;MMMMMMMMMMMMMMMMMMMMMMMM:
\x1b[91;1m:MMMMMMMMMMMMMMMMMMMMMMMM:
\x1b[91;1m.MMMMMMMMMMMMMMMMMMMMMMMMX.
\x1b[91;1mkMMMMMMMMMMMMMMMMMMMMMMMMWd.
\x1b[95;1m.XMMMMMMMMMMMMMMMMMMMMMMMMMMk
\x1b[95;1m .XMMMMMMMMMMMMMMMMMMMMMMMMK.
\x1b[94;1m   kMMMMMMMMMMMMMMMMMMMMMMd
\x1b[94;1m    ;KMMMMMMMWXXWMMMMMMMk.
\x1b[94;1m      .cooc,.    .,coo:.\x1b[0m
                ",
        ),
        "win11" => String::from(
            "
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll

\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll
\x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    ",
        ),
        "win" => String::from(
            "
\x1b[34m                  .oodMMMMMMMMMMMMM
\x1b[34m      ..oodMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34moodMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM

\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34m`^^^^^^MMMMMMM  MMMMMMMMMMMMMMMMMMM
\x1b[34m    ````^^^^  ^^MMMMMMMMMMMMMMMMMMM
\x1b[34m                   ````^^^^^^MMMMMM\x1b[0m
    ",
        ),
        "arch" => String::from(
            "
 \x1b[94;1m                 ##
 \x1b[94;1m                ####
 \x1b[94;1m               ######
 \x1b[94;1m              ########
 \x1b[94;1m             ##########
 \x1b[94;1m            ############
 \x1b[94;1m           ##############
 \x1b[94;1m          ################
 \x1b[94;1m         ##################
 \x1b[94;1m        ####################
 \x1b[94;1m       ######################
 \x1b[94;1m      #########      #########
 \x1b[94;1m     ##########      ##########
 \x1b[94;1m    ###########      ###########
 \x1b[94;1m   ##########          ##########
 \x1b[94;1m  #######                  #######
 \x1b[94;1m ####                          ####
 \x1b[94;1m###                              ###\x1b[0m",
        ),
        "redhat" => String::from(
            "
\x1b[31;1m
\x1b[31;1m             ..   .:..
\x1b[31;1m           .:::::::::::::..
\x1b[31;1m           :::::::::::::::::
\x1b[31;1m          :::::::::::::::::::
\x1b[31;1m          .::::::::::::::::::
\x1b[31;1m   .:::::   .:::::::::::::::::
\x1b[31;1m  .:::::::     ..::::::::::::.
\x1b[31;1m   :::::::::.         ....    .::.
\x1b[31;1m    .::::::::::..            .:::::
\x1b[31;1m      .::::::::::::::::::::::::::::.
\x1b[31;1m         .:::::::::::::::::::::::::
\x1b[31;1m             ..::::::::::::::::::.
\x1b[31;1m                  ....:::::...
\x1b[31;1m
",
        ),
        "linux" => String::from(
            "
\x1b[1m        a8888b.
\x1b[1m       d888888b.
\x1b[1m       8P\"YP\"Y88
\x1b[1m       8|o||o|88
\x1b[1m       8\x1b[93;1m'    .\x1b[0m88
\x1b[1m       8\x1b[93;1m`._.'\x1b[0m Y8.
\x1b[1m      d/      `8b.
\x1b[1m     dP        Y8b.
\x1b[1m    d8:       ::88b.
\x1b[1m   d8\"         'Y88b
\x1b[1m  :8P           :888
\x1b[1m   8a.         _a88P
\x1b[1m \x1b[93;1m._/\"Y\x1b[0maa     .\x1b[93;1m|\x1b[0m 88P\x1b[93;1m|\x1b[0m
\x1b[1m \x1b[93;1m\\    Y\x1b[0mP\"    `\x1b[93;1m|     `.\x1b[0m
\x1b[1m \x1b[93;1m/     \\\x1b[0m.___.d\x1b[93;1m|    .'\x1b[0m
\x1b[93;1m `--..__)     `._.'\x1b[0m
    ",
        ),
        _ => String::from(""),
    }
}
