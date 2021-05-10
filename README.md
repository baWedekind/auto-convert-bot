# auto-convert-bot
This is a Telegram Bot for use mainly in group chats, that will parse all messages in the chat  
(excluding messages from other bots or to other bots) and detect and convert units of measurement.  
For example when discussing the weather someone might use Fahrenheit, and the bot will return the  
converted value in Celsius, or vice versa, meaning that neither the person wishing to simply tell  
people what their weather is like, nor the people who wish to understand it, needs to look up the  
converted value themselves. The bot can be found at
 [`@auto_convert_bot`](https://t.me/auto_convert_bot) on Telegram.  

<img src="resources/close-up-photo-of-yellow-tape-measure-3143085.jpg" alt="yellow tape" width="300"/>  

Profile Photo by [Marta Longas](https://www.pexels.com/@marta-longas-1449108?utm_content=attributionCopyText&utm_medium=referral&utm_source=pexels) from [Pexels](https://www.pexels.com/photo/close-up-photo-of-yellow-tape-measure-3143085/?utm_content=attributionCopyText&utm_medium=referral&utm_source=pexels)


Originally made as part of a university module in _Programming in Rust_.  

## Simple Dictionary implementation.
Based on time, complexity and efficiency, the task set by myself and the professor and tutor was to implement  
Auto Convert Bot using a single dictionary that would map units to their conversion unit with their conversion  
rates. This method, as opposed to a more object oriented approach, has the drawback of not allowing for change  
of conversion partners as easily, or allowing for dynamic choice of conversion partner, as units exist mainly  
as a list of names without any further type architecture. On the other hand it meant not getting bogged down  
with painstakingly defining Dimensions, Units, From and To conversions of all sorts, which could have ended up  
taking longer than it needed to, and been very repetitive, thereby proving little about my ability with the  
Rust Programming Language, which was the main aim.  
I have planned to add the ability of adding new conversion pairs or changing a unit's 'conversion partner' if the  
first unit already existed with:
```
/add_conversion@auto_convert_bot <unit1> <unit2> <conversion_rate>
```
This command is admin only as it requires overwriting of the (group)chat's instanced dictionary, as well as  
the user deciding what a correct conversion would be. Admins would have the possibility of creating their  
own units if they wished, and the power to either keep their dictionary clean and correct, or mess up the  
conversion values completely. For this reason I have planned to implement a second admin command:
```
/restore_defaults@auto_convert_bot
```
which will reset the dictionary to its default setting, as viewable in `lib.rs`.

For developers: it is also easy to extend functionality of this bot using this dictionary: just add an  
entry with the units you would like to see included by default and make a Pull Request! Alternatively,  
you can always clone the bot and make your own rules if you don't feel like waiting on my decisions.

## Self evaluation
I would say the core functionality of the bot works well, and although the conversions from `ChatSettings`  
to the database entries and back are wordy and complicated, they work, and are robust against double  
inserts. The [Diesel](http://diesel.rs/) ORM crate played a great part in making this possible by  
enabling me to write most of what I needed in Rust, instead of Postgresql.  
My reason for wanting a database was simply the need for individualised settings on a per chat basis,  
which means data persistence of some sort, or those settings would be gone every time the bot restarts.  


### Fuzzy Matching
The decision was made to make the bot use [the Sublime Fuzzy crate](https://crates.io/crates/sublime_fuzzy) to allow for  
misspellings or typos from users, since fuzzy regex is not (currently) available in rust. The alternative  
was to use regex and make the list of strings to search for per unit longer, although my reasoning was  
this would never cover nearly as many possibilities as an easily configurable fuzzy matching search.  
Sadly, this ended up necessitating a word by word iterative approach with lookaheads and lookbehinds which  
not only robbed me of a lot of time, but also turns out not to be able to do what I meant for it to do.  
Only with one week left in the project did I realise this crate only even returns a positive result if the  
exact pattern, in the exact order specified but possibly with extra characters before in-between and after  
it in was found in the search string. (eg. pattern "kilometre", search string "kilometer", incomplete match  
"**kilomet**e**r**" => returns `None`.) As such it does not catch even singular switched characters, both  
the most common clerical error and a correct alternative spelling in the case of all "metre/meter" based  
units.  

This was noticed too late in development to change, so unit recognition is not as robust as intended. The  
plan going forward with this bot as a personal project is to replace the sublime_fuzzy crate with regex  
matching, and working with that as well as possible, thereby significantly reducing code and runtime, as  
well as allowing for usage of fuzzy logic in regex should anyone implement it. (A beginning of an attempt  
seems to exist at [fuzzy-regex-rs](https://github.com/8573/fuzzy-regex.rs), but the project seems to have  
been laid to rest soon after it was initialized. Still, interest seems to exist.)  Despite the less robust  
matching on long_hands, official symbols for units are matched exactly, due to their exact nature, and  
therefore robust matching of units _is_ offered over the scientific symbols.

### Rust code
This project is very back end heavy in nature, consisting mainly of an entity (database) and model layer,  
with a complicated data structure from the entity layer perspective, all of which means very lengthy  
and pragmatic code, many would say untidy. Some effort could be made to split the incredibly long lib.rs  
into more modules, with their own rust files, but the main problem is the `Default` constructor for  
`ChatSettings`, which accounts for 936/~1500 lines, because it contains a hardcoded declaration of the  
unit set and dictionary, which will be moved to a config file in the future.  
The Rest of the code is utilitarian, `fetch_chat_settings` and `insert_chat_settings` both involve iterating  
over a complete collection of chatsettings, once as vector of tuples representing a complete join of database  
entries filtered by current chat id, and once the `ChatSettings` itself of course. `insert_chat_settings`  
could be split into multiple sub-functions more easily, but this is another beautification I ran out of time  
for.  