pub fn get_logo_by_distro(sys_name: &str) -> String {
    match sys_name {
        "deb" => {
            let logo = String::from(
                "
\x1b[1m    _,.ss$$$$$gg.          
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
            );
            logo
        }
        "ubuntu" => {
            let logo = String::from(
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
            );
            logo
        }
        "fedora" => {
            let logo = String::from(
                "
            
            .',;::::;,'.               
            .';:cccccccccccc:;,.            
         .;cccccccccccccccccccccc;.        
       .:cccccccccccccccccccccccccc:.        
     .;ccccccccccccc;.:dddl:.;ccccccc;.     
    .:ccccccccccccc;OWMKOOXMWd;ccccccc:.    
   .:ccccccccccccc;KMMc;cc;xMMc:ccccccc:.   
   ,cccccccccccccc;MMM.;cc;;WW::cccccccc,   
   :cccccccccccccc;MMM.;cccccccccccccccc:   
   :ccccccc;oxOOOo;MMM0OOk.;cccccccccccc:  
   cccccc:0MMKxdd:;MMMkddc.;cccccccccccc; 
   ccccc:XM0';cccc;MMM.;cccccccccccccccc'   
   ccccc;MMo;ccccc;MMW.;ccccccccccccccc;    
   ccccc;0MNc.ccc.xMMd:ccccccccccccccc;     
   cccccc;dNMWXXXWM0::cccccccccccccc:,      
   cccccccc;.:odl:.;cccccccccccccc:,.        
   :cccccccccccccccccccccccccccc:'.       
   .:cccccccccccccccccccccc:;,..
     '::cccccccccccccc::;,.        
     ",
            );
            logo
        }
        "win11" => {
            let logo = String::from(
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
            );
            logo
        }
        "win" => {
            let logo = String::from(
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
            );
            logo
        }
        _ => String::from(""),
    }
}
