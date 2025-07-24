use phf::phf_map;
use strum_macros::{self, Display, EnumCount, EnumString};

// These are the supported logos. To add a new logo,
// 1. Add a variant to this enum
// 2. Add a corresponding entry to the `LOGOS` map.
//    The key of the entry will be a `&'static str` that represents the name of
//    the enum variant, preserving the case, e.g. the variant `Logo::MacOS`
//    corresponds to `"MacOs"`.
#[derive(Debug, Display, PartialEq, Eq, EnumString, EnumCount)]
pub enum Logo {
    Deb,
    Ubuntu,
    Fedora,
    Mac,
    Win11,
    Win,
    Arch,
    Redhat,
    Void,
    Linux,
    Bsd,
}

static LOGOS: phf::Map<&'static str, &'static str> = phf_map! {
    "Deb" => "
    \x1b[1m       _,.ss$$$$$gg.\x1b[0m
    \x1b[1m    ,g$$$$$$$$$$$$$$$P.\x1b[0m
    \x1b[1m  ,g$$P`'     ```Y$$.`.\x1b[0m
    \x1b[1m ,$$P''              `$$$.\x1b[0m
    \x1b[1m',$$P       ,ggs.     `$$b:\x1b[0m
    \x1b[1m`d$$'     ,$P`'   \x1b[91;1m.\x1b[0m    $$$\x1b[0m
    \x1b[1m$$P'      d$'     \x1b[91;1m,\x1b[0m    $$P\x1b[0m
    \x1b[1m$$:      $$.   \x1b[91;1m-\x1b[0m    ,d$$`\x1b[0m
    \x1b[1m$$;      Y$b._   _,d$P'\x1b[0m
    \x1b[1mY$$.    \x1b[91;1m`.\x1b[0m``Y$$$$P`'\x1b[0m
    \x1b[1m`$$b      \x1b[91;1m`\"-.__\x1b[0m
    \x1b[1m `Y$$\x1b[0m
    \x1b[1m  `Y$$.\x1b[0m
    \x1b[1m    `$$b.\x1b[0m
    \x1b[1m       `Y$$b.\x1b[0m
    \x1b[1m          ``Y$b._\x1b[0m
    \x1b[1m              ```\x1b[0m",
    "Ubuntu" => "
    \x1b[1m                        \x1b[91;1m./+o+-\x1b[0m
    \x1b[1m                yyyyy- \x1b[91;1m-yyyyyy+\x1b[0m
    \x1b[1m             ://+//////\x1b[91;1m-yyyyyyo\x1b[0m
    \x1b[1m         \x1b[93;1m.++\x1b[0m .:/++++++/-.+sss/`\x1b[0m
    \x1b[1m       \x1b[93;1m.:++o:\x1b[0m  /++++++++/:--:/-\x1b[0m
    \x1b[1m      \x1b[93;1mo:+o+:++.\x1b[0m`..```.-/oo+++++/\x1b[0m
    \x1b[1m     \x1b[93;1m.:+o:+o/.\x1b[0m          `+sssoo+/\x1b[0m
    \x1b[1m.++/+:\x1b[93;1m+oo+o:`\x1b[0m             /sssooo.\x1b[0m
    \x1b[1m/+++//+:\x1b[93;1m`oo+o\x1b[0m               /::--:.\x1b[0m
    \x1b[1m\\+/+o+++\x1b[93;1m`o++o\x1b[0m               \x1b[91;1m++////.\x1b[0m
    \x1b[1m .++.o+\x1b[93;1m++oo+:`\x1b[0m             \x1b[91;1m/dddhhh.\x1b[0m
    \x1b[1m      \x1b[93;1m.+.o+oo:.\x1b[0m          \x1b[91;1m`oddhhhh+\x1b[0m
    \x1b[1m       \x1b[93;1m\\+.++o+o`\x1b[0m\x1b[91;1m`-````.:ohdhhhhh+\x1b[0m
    \x1b[1m        \x1b[93;1m`:o+++\x1b[0m \x1b[91;1m`ohhhhhhhhyo++os:\x1b[0m
    \x1b[1m          \x1b[93;1m.o:\x1b[0m`\x1b[91;1m.syhhhhhhh/\x1b[0m\x1b[93;1m.oo++o`\x1b[0m
    \x1b[1m              \x1b[91;1m/osyyyyyyo\x1b[93;1m++ooo+++/\x1b[0m
    \x1b[1m                  \x1b[91;1m`````\x1b[0m \x1b[93;1m+oo+++o\\:\x1b[0m
    \x1b[1m                         \x1b[93;1m`oo++.\x1b[0m",
    "Fedora" => "
    \x1b[94;1m             .',;::::;,'.\x1b[0m
    \x1b[94;1m         .';:cccccccccccc:;,.\x1b[0m
    \x1b[94;1m      .;cccccccccccccccccccccc;.\x1b[0m
    \x1b[94;1m    .:cccccccccccccccccccccccccc:.\x1b[0m
    \x1b[94;1m  .;ccccccccccccc;\x1b[0m.:dddl:.\x1b[94;1m;ccccccc;.\x1b[0m
    \x1b[94;1m .:ccccccccccccc;\x1b[0mOWMKOOXMWd\x1b[94;1m;ccccccc:.\x1b[0m
    \x1b[94;1m.:ccccccccccccc;\x1b[0mKMMc\x1b[94;1m;cc;\x1b[0mxMMc\x1b[94;1m:ccccccc:.\x1b[0m
    \x1b[94;1m,cccccccccccccc;\x1b[0mMMM.\x1b[94;1m;cc;\x1b[0m;WW:\x1b[94;1m:cccccccc,\x1b[0m
    \x1b[94;1m:cccccccccccccc;\x1b[0mMMM.\x1b[94;1m;cccccccccccccccc:\x1b[0m
    \x1b[94;1m:ccccccc;\x1b[0moxOOOo\x1b[94;1m;\x1b[0mMMM0OOk.\x1b[94;1m;cccccccccccc:\x1b[0m
    \x1b[94;1mcccccc:\x1b[0m0MMKxdd:\x1b[94;1m;\x1b[0mMMMkddc.\x1b[94;1m;cccccccccccc;\x1b[0m
    \x1b[94;1mccccc:\x1b[0mXM0'\x1b[94;1m;cccc;\x1b[0mMMM.\x1b[94;1m;cccccccccccccccc'\x1b[0m
    \x1b[94;1mccccc;\x1b[0mMMo\x1b[94;1m;ccccc;\x1b[0mMMW.\x1b[94;1m;ccccccccccccccc;\x1b[0m
    \x1b[94;1mccccc;\x1b[0m0MNc.\x1b[94;1mccc\x1b[0m.xMMd\x1b[94;1m:ccccccccccccccc;\x1b[0m
    \x1b[94;1mcccccc;\x1b[0mdNMWXXXWM0:\x1b[94;1m:cccccccccccccc:,\x1b[0m
    \x1b[94;1mcccccccc;\x1b[0m.:odl:.\x1b[94;1m;cccccccccccccc:,.\x1b[0m
    \x1b[94;1m:cccccccccccccccccccccccccccc:'.\x1b[0m
    \x1b[94;1m.:cccccccccccccccccccccc:;,..\x1b[0m
    \x1b[94;1m  '::cccccccccccccc::;,.\x1b[0m",
    "Mac" => "
    \x1b[92;1m                   'c.\x1b[0m
    \x1b[92;1m                ,xNMM.\x1b[0m
    \x1b[92;1m              .OMMMMo\x1b[0m
    \x1b[92;1m              OMMM0,\x1b[0m
    \x1b[92;1m    .;loddo:' loolloddol;.\x1b[0m
    \x1b[92;1m  cKMMMMMMMMMMNWMMMMMMMMMM0:\x1b[0m
    \x1b[93;1m.KMMMMMMMMMMMMMMMMMMMMMMMWd.\x1b[0m
    \x1b[93;1mXMMMMMMMMMMMMMMMMMMMMMMMX.\x1b[0m
    \x1b[91;1m;MMMMMMMMMMMMMMMMMMMMMMMM:\x1b[0m
    \x1b[91;1m:MMMMMMMMMMMMMMMMMMMMMMMM:\x1b[0m
    \x1b[91;1m.MMMMMMMMMMMMMMMMMMMMMMMMX.\x1b[0m
    \x1b[91;1mkMMMMMMMMMMMMMMMMMMMMMMMMWd.\x1b[0m
    \x1b[95;1m.XMMMMMMMMMMMMMMMMMMMMMMMMMMk\x1b[0m
    \x1b[95;1m .XMMMMMMMMMMMMMMMMMMMMMMMMK.\x1b[0m
    \x1b[94;1m   kMMMMMMMMMMMMMMMMMMMMMMd\x1b[0m
    \x1b[94;1m    ;KMMMMMMMWXXWMMMMMMMk.\x1b[0m
    \x1b[94;1m      .cooc,.    .,coo:.\x1b[0m",
    "Win11" => "
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m

    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m
    \x1b[34;1mlllllllllllllll   lllllllllllllll\x1b[0m",
    "Win" => "
    \x1b[34m                  .oodMMMMMMMMMMMM\x1b[0m
    \x1b[34m      ..oodMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34moodMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m

    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34mMMMMMMMMMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34m`^^^^^^MMMMMMM  MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34m    ````^^^^  ^^MMMMMMMMMMMMMMMMMM\x1b[0m
    \x1b[34m                  ````^^^^^^MMMMMM\x1b[0m",
    "Arch" => "
    \x1b[94;1m                 ##                 \x1b[0m
    \x1b[94;1m                ####                \x1b[0m
    \x1b[94;1m               ######               \x1b[0m
    \x1b[94;1m              ########              \x1b[0m
    \x1b[94;1m             ##########             \x1b[0m
    \x1b[94;1m            ############            \x1b[0m
    \x1b[94;1m           ##############           \x1b[0m
    \x1b[94;1m          ################          \x1b[0m
    \x1b[94;1m         ##################         \x1b[0m
    \x1b[94;1m        ####################        \x1b[0m
    \x1b[94;1m       ######################       \x1b[0m
    \x1b[94;1m      #########      #########      \x1b[0m
    \x1b[94;1m     ##########      ##########     \x1b[0m
    \x1b[94;1m    ###########      ###########    \x1b[0m
    \x1b[94;1m   ##########          ##########   \x1b[0m
    \x1b[94;1m  #######                  #######  \x1b[0m
    \x1b[94;1m ####                          #### \x1b[0m
    \x1b[94;1m###                              ###\x1b[0m",
    "Redhat" => "
    \x1b[31;1m\x1b[0m
    \x1b[31;1m             ..   .:..\x1b[0m
    \x1b[31;1m           .:::::::::::::..\x1b[0m
    \x1b[31;1m           :::::::::::::::::\x1b[0m
    \x1b[31;1m          :::::::::::::::::::\x1b[0m
    \x1b[31;1m          .::::::::::::::::::\x1b[0m
    \x1b[31;1m   .:::::   .:::::::::::::::::\x1b[0m
    \x1b[31;1m  .:::::::     ..::::::::::::.\x1b[0m
    \x1b[31;1m   :::::::::.         ....    .::.\x1b[0m
    \x1b[31;1m    .::::::::::..            .:::::\x1b[0m
    \x1b[31;1m      .::::::::::::::::::::::::::::.\x1b[0m
    \x1b[31;1m         .:::::::::::::::::::::::::\x1b[0m
    \x1b[31;1m             ..::::::::::::::::::.\x1b[0m
    \x1b[31;1m                  ....:::::...\x1b[0m
    \x1b[31;1m\x1b[0m",
    "Void" => "
    \x1b[32;1m                ...:::::::...\x1b[0m
    \x1b[32;1m             .:::::::::::::::::.\x1b[0m
    \x1b[32;1m            '::::::::::::::::::::.\x1b[0m
    \x1b[32;1m        .     '::::''   '':::::::::.\x1b[0m
    \x1b[32;1m       .:V.     '           '':::::::\x1b[0m
    \x1b[32;1m      .VVVVV:.                '::::::.\x1b[0m
    \x1b[32;1m     .VVVVVV'      .:::::.     '::::::.\x1b[0m
    \x1b[90;1m######\x1b[32;1mVVVVV: \x1b[90;1m##^ v########v  ####\x1b[32;1m:\x1b[90;1m##########v\x1b[0m
    \x1b[90;1m v#####\x1b[32;1mVVVV\x1b[90;1m##^ v####\x1b[32;1m:::\x1b[90;1m####\x1b[32;1m.\x1b[90;1m#### ####\x1b[32;1m:::\x1b[90;1m#####\x1b[0m
    \x1b[90;1m  v########^   ####\x1b[32;1m:::\x1b[90;1m####\x1b[32;1m:\x1b[90;1m#### #####\x1b[32;1m::\x1b[90;1m#####^\x1b[0m
    \x1b[90;1m   v####^\x1b[32;1mVV:   \x1b[90;1m^########^\x1b[32;1m:\x1b[90;1m#### ###########^\x1b[0m
    \x1b[32;1m     VVVVVVV.      ':::::'      :::::::\x1b[0m
    \x1b[32;1m      VVVVVVV.                 '::::::\x1b[0m
    \x1b[32;1m       VVVVVVVV..          .     ':::\x1b[0m
    \x1b[32;1m        'VVVVVVVVVV::...::VVV.     '\x1b[0m
    \x1b[32;1m          'VVVVVVVVVVVVVVVVVVV:.\x1b[0m
    \x1b[32;1m            ':VVVVVVVVVVVVVVVVV:'\x1b[0m
    \x1b[32;1m               '':VVVVVVVVV:''\x1b[0m",
    "Linux" => "
    \x1b[1m        a8888b.\x1b[0m
    \x1b[1m       d888888b.\x1b[0m
    \x1b[1m       8P\"YP\"Y88\x1b[0m
    \x1b[1m       8|o||o|88\x1b[0m
    \x1b[1m       8\x1b[93;1m'    .\x1b[0m88
    \x1b[1m       8\x1b[93;1m`._.'\x1b[0m Y8.
    \x1b[1m      d/      `8b.\x1b[0m
    \x1b[1m     dP        Y8b.\x1b[0m
    \x1b[1m    d8:       ::88b.\x1b[0m
    \x1b[1m   d8\"         'Y88b\x1b[0m
    \x1b[1m  :8P           :888\x1b[0m
    \x1b[1m   8a.         _a88P\x1b[0m
    \x1b[1m \x1b[93;1m._/\"Y\x1b[0maa     .\x1b[93;1m|\x1b[0m 88P\x1b[93;1m|\x1b[0m
    \x1b[1m \x1b[93;1m\\    Y\x1b[0mP\"    `\x1b[93;1m|     `.\x1b[0m
    \x1b[1m \x1b[93;1m/     \\\x1b[0m.___.d\x1b[93;1m|    .'\x1b[0m
    \x1b[93;1m `--..__)     `._.'\x1b[0m",
    "Bsd" => "
    \x1b[1m                ,        ,         \x1b[0m
    \x1b[1m               /(        )`        \x1b[0m
    \x1b[1m               \\ \\___   / |        \x1b[0m
    \x1b[1m               /- _  `-/  '        \x1b[0m
    \x1b[1m              (/\\/ \\ \\   /\\        \x1b[0m
    \x1b[1m              / /   | `    \\       \x1b[0m
    \x1b[1m              O O   ) /    |       \x1b[0m
    \x1b[1m              `-^--'`<     '       \x1b[0m
    \x1b[1m             (_.)  _  )   /        \x1b[0m
    \x1b[1m              `.___/`    /         \x1b[0m
    \x1b[1m                `-----' /          \x1b[0m
    \x1b[1m   <----.     __ / __   \\          \x1b[0m
    \x1b[1m   <----|====O)))==) \\) /====|     \x1b[0m
    \x1b[1m   <----'    `--' `.__,' \\         \x1b[0m
    \x1b[1m                |        |         \x1b[0m
    \x1b[1m                 \\       /       /\\\x1b[0m
    \x1b[1m            ______( (_  / \\______/ \x1b[0m
    \x1b[1m          ,'  ,-----'   |          \x1b[0m
    \x1b[1m          `--{__________)\x1b[0m",
};

pub fn get_logo_by_distro(logo: Logo) -> String {
    LOGOS[&logo.to_string()].to_string()
}

mod test {
    #[test]
    fn test_logos_keys_match_enum() {
        use std::str::FromStr;
        use strum::EnumCount;
        assert!(
            super::LOGOS.len() == super::Logo::COUNT,
            "The number of LOGOS keys is not the same as the number of Logo variants",
        );
        super::LOGOS.keys().for_each(|key| {
            assert!(
                super::Logo::from_str(key).is_ok(),
                "LOGOS key {:?} is not a Logo enum variant",
                key,
            );
        });
    }

    #[test]
    fn test_logos_reset_sgr() {
        super::LOGOS.entries().for_each(|(name, logo)| {
            logo.lines().enumerate().for_each(|(line_no, l)| {
                let sgr_only = crate::ansi::truncate(l, 0);
                if sgr_only.is_empty() {
                    return;
                }

                assert!(
                    sgr_only.ends_with("\x1b[0m"),
                    "line {} of logo {:?} did not reset SGR. Please terminate it with \"\\x1b[0m\"",
                    line_no,
                    name,
                );
            });
        });
    }
}
