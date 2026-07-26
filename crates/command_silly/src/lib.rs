// Goober Bot, Discord bot
// Copyright (C) 2025  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use commands_shared::CustomData;
use emoji::*;
use poise::{
    CreateReply, command,
    serenity_prelude::{CreateAllowedMentions, Mentionable, UserId},
};
use rand::{rng, seq::IteratorRandom};
use shared::Context;

/// ```
/// silly_command!(
///     /// Command description
///     #[early_access] // Optional line
///     fn command_name("User description") {
///         bot_message = "Message when used on bot, must include {author}";
///         author_message = "Message when used on author, must include {author}";
///         messages = [
///             "Random messages to choose from, must include {author} and {user}",
///             "Messages to randomly choose from, must include {author} and {user}",
///         ];
///     }
/// );
/// ```
macro_rules! silly_command {
    (
        #[doc = $doc:expr]
        #[early_access]
        $(#[command($($command_extra:expr),+)])?
        fn $name:ident($user_description:literal) {
            bot_message = $bot_message:literal;
            author_message = $author_message:literal;
            messages = [
                $($message:literal),+$(,)?
            ];
        }
    ) => {
        silly_command! {
            #[doc = $doc]
            #[command(custom_data = "CustomData { early_access: true }"$(, $($command_extra),+)?)]
            fn $name($user_description) {
                bot_message = $bot_message;
                author_message = $author_message;
                messages = [
                    $($message),+,
                ];
            }
        }
    };
    (
        #[doc = $doc:expr]
        $(#[command($($command_extra:expr),+)])?
        fn $name:ident($user_description:literal) {
            bot_message = $bot_message:literal;
            author_message = $author_message:literal;
            messages = [
                $($message:literal),+$(,)?
            ];
        }
    ) => {
        #[doc = $doc]
        #[command(
            slash_command,
            category = "Silly",
            install_context = "Guild|User",
            interaction_context = "Guild|BotDm|PrivateChannel",
            required_bot_permissions = "USE_EXTERNAL_EMOJIS",
            $($($command_extra),+)?
        )]
        pub async fn $name(
            ctx: Context<'_>,
            #[description = $user_description] user: UserId,
        ) -> Result<(), poise_error::anyhow::Error> {
            let content;

            if user == ctx.framework().bot_id {
                content = format!($bot_message, author = ctx.author().mention());
            } else if user == ctx.author().id {
                content = format!($author_message, author = ctx.author().mention());
            } else {
                let mut rng = rng();

                content = [
                    $(format!(
                        $message,
                        author = ctx.author().mention(),
                        user = user.mention(),
                    )),+
                ]
                .into_iter()
                .choose(&mut rng)
                .expect("list of possible message content should not be empty");
            }

            ctx.send(
                CreateReply::default()
                    .content(content)
                    .allowed_mentions(CreateAllowedMentions::new().users([user])),
            )
            .await?;

            Ok(())
        }
    };
}

silly_command! {
    /// Boops a being :3c
    fn boop("Your victim >:3") {
        bot_message = "I have been booped by {author} <:floofOwO:1530411511481110548>";
        author_message = "{author} just booped themselves... that's a little sad, won't someone else boop them? <:floofSad:1530411572961476671>";
        messages = [
            "{author} booped {user}!!! <:floofOwO:1530411511481110548>",
            "{user} just got booped by {author}?? <:floofLoadAnimated:1530411068063617096>",
            "Lmao I just saw {author} boop {user} <:floofLol:1530411409765171290>",
            "Dear {user},\n\nGet booped, nerd. <:floofSmug:1530411628431147188>\n\nSincerely, {author}",
            "{author} booped {user}, I think they're trying to pick a fight <:floofNervous:1530411474776887366>",
        ];
    }
}

silly_command! {
    /// Embrace the bobin within us all and gnaw on one's bones
    fn gnaw("The subject of today's gnawing") {
        bot_message = "GRAAAHH {author} STOP GNAWING MY BONES GET OFF HELP <:floofScared:1530411585841926289>";
        author_message = "{author}'s gnawing on... their own bones? Are they good...? <:floofLoadAnimated:1530411068063617096>";
        messages = [
            "{author} is gnawing on {user}'s bones <:floofNom:1530411497660878938>",
            "{author} craves the bones of {user} <:floofNom:1530411497660878938>",
            "{author} hungers for the bones of a {user} <:floofNom:1530411497660878938>",
            "Hey uh, {user}, did you know there's a {author} gnawing on your bones? <:floofLurk:1530411419634503680>",
        ];
    }
}

silly_command! {
    /// You wild animal...
    fn bite("That tame person...") {
        bot_message = "BAH- {author}'S BITING ME- GET IT OFF GET IT OFF- <:floofScared:1530411585841926289>";
        author_message = "{author} bit themselves... why'd they do that? <:floofLoadAnimated:1530411068063617096>";
        messages = [
            "D- did {author} just bite {user}?? <:floofOwO:1530411511481110548>",
            "The intrusive thoughts won and now {author}'s biting {user} <:floofMischief:1530411443587911680>",
            "\\*CHOMP\\*\n{author} bit {user} <:floofNom:1530411497660878938>",
            "Oh, sorry {user}, I guess {author} got hungry <:floofTired:1530411654976901250>",
            "Eeeeek! {user}, didn't you know? *{author} bites!* <:floofNervous:1530411474776887366>",
            "{user} got a little too close to {author} and got bit <:floofNervous:1530411474776887366>",
        ];
    }
}

silly_command! {
    /// You know what you are
    fn meow("Get their attention") {
        bot_message = "Hm? What's that {author}? Oh I see... mhm... okay, okay, I understand <:floofCat:1530411304186151103>";
        author_message = "{author} is meowing at themselves lol, schizophrenic cat <:floofCat:1530411304186151103>";
        messages = [
            "Uhh, {author} just meowed at {user} <:floofWhat:1530411667463340144>",
            "{author} is a furry and they want {user} to know it <:floofMischief:1530411443587911680>",
            "{author} is so silly, they think {user} can understand their meowing <:floofLol:1530411409765171290>",
            "{user}, be afraid... {author} is meowing at you <:floofPeek:1530411538966384691>",
            "{user}, {author} is meowing at you, won't you give them what they want? <:floofPlead:1530411552975360231>",
            "{user}, I have a message for you: \"meow meow meow meow meow meow meow meow\"\n{author} gave it to me <:floofHappy:1530411341180047400>",
            "{author} just *nya*-ed all over the place- {user}, clean this up! <:floofWhat:1530411667463340144>",
            "{user}... sire... I have a message for you, from {author}... \\*ahem\\*... \"meow meow meow, meow meow, meow meow meow meow meow, meow!\"\nI'm just the messenger please don't hurt me <:floofNervous:1530411474776887366>"
        ];
    }
}

silly_command! {
    /// MURRRRRDEERRRRRRRRRRR
    fn murder("KILL THEM KILL THEM KILL THEM >:D") {
        bot_message = "GAH {author} HAS A KNIFE AND IS RUNNING AT ME WAAAA <:floofScared:1530411585841926289>";
        author_message = "BAH- {author} JUST K-KILLED THEMSELVES??? NOOOOOOOOOO <:floofScared:1530411585841926289>";
        messages = [
            "{author} crept up behind {user} and murdered them!!! <:floofOwO:1530411511481110548>",
            "{author} just pulled out a bazooka and blew {user} up!?!? <:floofOwO:1530411511481110548>",
            "{author} stared directly into {user}'s eyes and shouted \"POMEGRANATE\", triggering the cognitohazard previously planted in {user}'s brain, killing them instantly <:floofNervous:1530411474776887366>",
            "{author} just went \"BOO\", giving {user} a fatal heart attack <:floofOwO:1530411511481110548>",
            "{author} just went \"OOGA BOOGA\", giving {user} a fatal heart attack <:floofOwO:1530411511481110548>",
            "{author} killed {user} when the lights went out so no one would know it was them... <:floofSmug:1530411628431147188>",
        ];
    }
}

silly_command! {
    /// Let them know that they're a good being :>
    fn pat("Good being in question") {
        bot_message = "Awawawawa {author} gave me a pat pat on the head <:floofPat:1530411525339090954>";
        author_message = "Aw, {author} pat themselves on the head, won't someone else give them a little pat? <:floofPlead:1530411552975360231>";
        messages = [
            "{author} gave {user} a little pat on the head <:floofPat:1530411525339090954>",
            "{author} wants {user} to know they are a good being by giving them a pat on the head <:floofPat:1530411525339090954>",
            "{user} got pat on the head by {author} <:floofPat:1530411525339090954>",
            "{user} has been selected to receive a soothing pat on the head from {author} <:floofPat:1530411525339090954>",
        ];
    }
}

silly_command! {
    /// 😳
    fn kiss("Omg who is it who is it???") {
        bot_message = "\\*gasp* oh- oh my goodness- {author} kissed me!!! <:floofWoozy:1530411680419283014>";
        author_message = "{author} kissed themselves? ...how? <:floofWhat:1530411667463340144>";
        messages = [
            "AWWWWWWWWW- {author} gave {user} a kiss!!!! <:floofPlead:1530411552975360231>",
            "Hehehehe, {author} gave {user} a little smooooch <:floofHappy:1530411341180047400>",
            "OMG- GUYS- {author} JUST KISSED {user}!!! <:floofPlead:1530411552975360231>",
            "Hehehe {author} and {user} are so cute, they just kissed each other <:floofHappy:1530411341180047400>",
            "{author} **VIOLENTLY** pulled {user} to them and **SMOOCHED** them on the **LIPS**, not letting **ANYONE ELSE** in <:floofMischief:1530411443587911680>",
        ];
    }
}

silly_command! {
    /// Doesn't this count as necromancy?
    fn revive("The deceased") {
        bot_message = "What- I- {author}, I'm not dead- <:floofWhat:1530411667463340144>";
        author_message = "... Oh my god *{author}'S **IMMORTAL**-* <:floofScared:1530411585841926289>";
        messages = [
            "{author} performed necromancy on {user}, now they're a *zOoOmBiIeE oOoOo* <:floofSmug:1530411628431147188>",
            "{author} crouched by {user} and held `E` for a few seconds <:floof:1530411269155455129>",
            "{author} graciously donated a health pack to {user} <:floofHappy:1530411341180047400>",
            "{author} performed a ritual and sacrificed a lamb to bring {user} back to life <:floofOwO:1530411511481110548>",
            "In a flash of light, {author} descended upon {user} and gave them the gift of another life <:floofInnocent:1530411361803305081>",
        ];
    }
}

silly_command! {
    /// Does somebody want a huuuug :3
    fn hug("No seriously, who wants a hug? I need to know-") {
        bot_message = "Awawawa, thanks for the hug {author} <:floofHeart:1530411352336634059>";
        author_message = "{author} gave themselves a hug <:floofPlead:1530411552975360231>";
        messages = [
            "{author} gave {user} a much needed hug <:floofHeart:1530411352336634059>",
            "{author} wrapped their arms around {user} for a hug <:floofHeart:1530411352336634059>",
            "Awww, {author} and {user} are hugging, so wholesome <:floofPlead:1530411552975360231>",
            "{author} and {user} are hugging and uhh, they're really cute <:floofPlead:1530411552975360231>",
            "Before {user} could say anything, {author} had them trapped in an embrace <:floofHeart:1530411352336634059>",
            "{user} got absolutely loved with a hug by {author} <:floofHeart:1530411352336634059>",
        ];
    }
}

silly_command! {
    /// Knock some sense into somebody
    fn slap("Senseless somebody in question") {
        bot_message = "OW- HEY- {author} just *slapped* me, what the heck!? <:floofScared:1530411585841926289>";
        author_message = "{author} must think they're dreaming or something, they just slapped themselves <:floofOwO:1530411511481110548>";
        messages = [
            "{author} slapped {user} and shouted \"SNAP OUT OF IT\" <:floofLol:1530411409765171290>",
            "{author} tried to knock some sense into {user} by slapping them <:floofLol:1530411409765171290>",
            "{author} decided to slap {user} across the face <:floofOwO:1530411511481110548>",
            "{author} slapped {user}, just cuz they felt like it <:floofBlep:1530411290340753408>",
            "{user} find themselves facing the opposite direction after {author}'s slap turned them around <:floofOwO:1530411511481110548>",
            "In slapstick fashion, {author} slapped {user} causing them to comically spin in circles <:floofLol:1530411409765171290>",
        ];
    }
}

silly_command! {
    /// *bap* *bap*
    fn bap("Bap receiver") {
        bot_message = "WHAT- oh, {author} just bapped me. What do you want buddy? <:floofTired:1530411654976901250>";
        author_message = "{author}'s bapping themselves, they seem a little confused... <:floofTired:1530411654976901250>";
        messages = [
            "{author} bapped {user} and {user} jumped <:floofLol:1530411409765171290>",
            "{user} was startled for a moment when {author} snuck up and bapped them <:floofLol:1530411409765171290>",
            "{author} sloooowly reached out... and then bapped {user} <:floofLurk:1530411419634503680>",
            "{author} swat at and bapped {user} like a cat <:floofCat:1530411304186151103>",
            "LOOK OUT {user}, {author}'S GONNA- oh, they only bapped you <:floofTired:1530411654976901250>",
            "{author} bapped {user} before retreating into the shadows... <:floofPeek:1530411538966384691>",
        ];
    }
}

silly_command! {
    /// FNAF style
    fn jumpscare("Night guard") {
        bot_message = "# *AAAAAAAA-*\n*{author}... \\*gasp\\* **jumpscared** me... jeez...* <:floofScared:1530411585841926289>";
        author_message = "{author} looked in a mirror and went \"raaahhhh!\"\nVery scary <:floofSmug:1530411628431147188>";
        messages = [
            "{author} did the bite of '87 on {user} <:floofNom:1530411497660878938>",
            "{user} almost got to 6 AM but {author} got to them first <:floofPeek:1530411538966384691>",
            "{author} ran down the hall and before {user} could shut the door, it was game over <:floofPeek:1530411538966384691>",
            "{user} didn't catch {author} in time and they leapt from the shadows <:floofLurk:1530411419634503680>",
            "{user} forgot to wind the music box and {author} went straight to them <:floofMischief:1530411443587911680>",
            "{user} just realized that {author} had snuck up to them, but by then it was already too late <:floofPeek:1530411538966384691>",
            "***RAAAHHHH!!!***\n{author} jumpscared {user} <:floofMischief:1530411443587911680>",
        ];
    }
}

silly_command! {
    /// Yeah you know, just, carry a person
    fn carry("Who are you carrying away?") {
        bot_message = "Oh- {author} picked me up- okay, where're we going? <:floofHappy:1530411341180047400>";
        author_message = "{author} discovered a physics glitch and carried themselves into the sky... <:floofWhat:1530411667463340144>";
        messages = [
            "{author} just, grabbed {user} and started carrying them over their shoulder <:floofOwO:1530411511481110548>",
            "Oh my- {author} is now carrying {user}! <:floofOwO:1530411511481110548>",
            "{author} must be strong- they just picked {user} right up! <:floofOwO:1530411511481110548>",
            "Whoa! {author} picked up {user}- I wonder where {author}'s gonna take them? <:floofHappy:1530411341180047400>",
            "Hey! {author} just stole something and took off! They stole... {user}!! <:floofOwO:1530411511481110548>",
            "Awww {author}'s giving {user} a piggy back ride <:floofHappy:1530411341180047400>",
            "Pfft, {author} just started carrying {user} under {author}'s arm as if they were luggage <:floofLol:1530411409765171290>",
        ];
    }
}

silly_command! {
    /// Hey, hey, hey, hey, hey, hey, hey-
    fn poke("Whom to get the attention of") {
        bot_message = "Huh, oh- what do you need {author}? <:floof:1530411269155455129>";
        author_message = "{author} has... poked themselves? I'm not sure why... <:floofWhat:1530411667463340144>";
        messages = [
            "Hey {user}, hey {user}, hey {user}, hey {user}-\nI think {author} might want your attention <:floofBlep:1530411290340753408>",
            "\\*poke* \\*poke*\n{author}'s poking you, {user} <:floofBlep:1530411290340753408>",
            "{author} poked {user} with a stick from a distance <:floofPeek:1530411538966384691>",
            "{user} felt a sudden poke from {author}, startling them <:floofOwO:1530411511481110548>",
            "Sorry {user}, {author}'s not gonna stop poking you until you pay attention to them <:floofTired:1530411654976901250>",
            "{user} suddenly felt a poke on their shoulder, only to turn and see {author} innocently looking away <:floofInnocent:1530411361803305081>",
        ];
    }
}

silly_command! {
    /// Wh- HO- HAH HAH HAH- ST- STOP IT HAH-
    #[early_access]
    fn tickle("TICKLE SOMEBODY ELSE, PLEEAASE") {
        bot_message = "N- NO- HAH HAH- HEEE- HO- HAH- PLEASE- SOMEBODY- MAKE {author} STOPPPP HUHEHAH- <:floofScared:1530411585841926289>";
        author_message = "{author}- you can't tickle *yourself*, it won't work... <:floofTired:1530411654976901250>";
        messages = [
            "Oh no- {user}, {author}'s discovered your weakness: *tickles* <:floofNervous:1530411474776887366>",
            "The crime: {user}'s been *killed*\nThe criminal: {author}\nThe weapon: *tickles* <:floofSad:1530411572961476671>",
            "All it took from {author} was a little \\*tickle\\* \\*tickle\\* and {user} totally collapsed <:floofLol:1530411409765171290>",
            "WHOA, OH MY GOD, {author} TACKLED {user} TO THE GROUND AND- oh {author}'s just tickling them, they'll be fine <:floofHappy:1530411341180047400>",
            "{author} tried to tickle {user}, ***but they resisted...*** whoa, that's crazy, honestly <:floofOwO:1530411511481110548>",
            "Alright, so, {author}, you wanna take down {user}, huh? *Well I've got just the thing, one weird trick discovered by a mom, **tickles*** <:floofCool:1530411312956313680>",
            "Oof, {author} suddenly tickled {user}, but got punched in the face by {user}'s reflexes- ouch... <:floofLurk:1530411419634503680>"
        ];
    }
}

silly_command! {
    /// *Blows up pancakes with mind*
    #[early_access]
    fn blow_up(r#""Mah fricking pancakes""#) {
        bot_message = "{author} blew up Goober B... wait, wh- <:floofSplode:1530765283516612708>}";
        author_message = "***{author}'S SELF DESTRUCTING!!! <:floofScared:1530411585841926289>***";
        messages = [
            "{author} blew up {user} with their mind <:floofNervous:1530411474776887366>",
            "{author} snapped their fingers and {user} exploded <:floofNervous:1530411474776887366>",
            "{author} just, said \"blow up\", and then {user} blew up?? <:floofNervous:1530411474776887366>",
            "{user} was engulfed in an explosion, meanwhile {author} walked the other way, wearing sunglasses, of course <:floofCool:1530411312956313680>",
            "Oh no... {author} hit the big red button, and {user} *exploded* <:floofNervous:1530411474776887366>",
            "*{user}, get out of here! {author}'s gonna-* <:floofSplode:1530765283516612708>}",
            "Oops, {author} found out that the trigger word for the explosive implant in {user}'s head was \"petrichor\" <:floofNervous:1530411474776887366>"
        ];
    }
}

silly_command! {
    /// hamburger
    #[early_access]
    fn hamburger("hamburger") {
        bot_message = "{author} hamburger";
        author_message = "{author} hamburger";
        messages = [
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger <:floofSplode:1530765283516612708>",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger", 
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburger",
            "{author} {user} hamburber",
            "{user} {author} hamburger",
        ];
    }
}

silly_command! {
    /// What does "defenestrate" mean...?
    #[early_access]
    fn defenestrate("I guess you could just try it and see...") {
        bot_message = "Oh no! {author}'s defenestrating me! Which is a (rare) word which means to throw (someone) out of a window! Oh no!!! <:floofOwO:1530411511481110548>";
        author_message = "{author} needed out so they swiftly *jumped out of the window* to their immediate left. Seems a little bit rash to me <:floofTired:1530411654976901250>";
        messages = [
            "{author} defenestrated {user}. Haha you thought I was gonna tell you what the word meant hehe <:floofTeehee:1530411641596809246>",
            "> # de•fen•es•trate |dēˈfenəˌstrāt|\n> -# verb *[with object]*\n> 1. *rare* **throw (someone) out of a window**: *{author} defenestrated {user} in a case that was previously suspected to be suicide.*\n> 2. **remove or dismiss (someone) from a position of power or authority**: *the overwhelming view is that they should be defenestrated before the next election.*\n> \n> ### ORIGIN\n> **early 17th century (as *defenestrated*): see defenestration.**\n\nSo now you know! <:floofBlep:1530411290340753408>",
            "Oh, {author} threw {user} out of a window- so that's what it means... <:floofTired:1530411654976901250>",
            "{author} *threw* {user} *out of a window*- like, *breaking through the glass, {user} is now a puddle on the ground outside*! Idk how to express how ridiculous this is <:floofNervous:1530411474776887366>",
            "Hey {author}, where'd {user} go?\n \\*suspiciously {user} shaped hole in the window* <:floofLoadAnimated:1530411068063617096>",
            "Oopsies {author} just committed a crime on {user}! They pushed {user} out of the window of a 12 story building! Whoops! Your honor, I plead: I didn't know what the word meant!!! <:floofPlead:1530411552975360231>",
            "{author}, you just pushed {user} out of a window... don't play dumb, you knew what the word meant, you're a smart cookie- and a criminal <:floofWhat:1530411667463340144>",
        ];
    }
}

silly_command! {
    /// Stop in the name of the law!
    #[early_access]
    fn arrest("Anything you say can and will be used against you.") {
        bot_message = "PUT ME DOWN {author} I AM THE LAW {I_AM_THE_LAW}";
        author_message = "Uh, {author} just slapped on a pair of handcuffs... I guess they're arresting themselves <:floofTired:1530411654976901250>";
        messages = [
            "*Wee woo wee woo*, stop right there {user}, this is the Goober Police! We have warrant for you arrest, on behalf of {author} <:floofMischief:1530411443587911680>",
            "{user}, on behalf of {author}, you are under arrest! Anything you say can and will be used against you <:floofMischief:1530411443587911680>",
            "Whoa, it turns out {author} was an undercover cop the whole time- and now they're arresting {user} <:floofOwO:1530411511481110548>",
            "{author} picked up {user} and put them in baby jail <:floofTired:1530411654976901250>",
            "{author} sentenced {user} to a life behind bars... dang, that's rough <:floofSad:1530411572961476671>",
            "Hey there, uh, {user}? Is it? Yeah, uhm, we've got a noise complaint from {author} for you... we're gonna have to put you under arrest <:floofSmug:1530411628431147188>",
            "{author} is arresting {user}? Wait, they're not a cop- where'd they get those handcuffs? <:floofLoadAnimated:1530411068063617096>",
        ];
    }
}
silly_command! {
    /// This is what its all about!!
    #[early_access]
    fn about("about") {
        bot_message = "fork of a fork, slightly altered bot by @baxter.zip, fork of vgskye's fork of valentinegbs goober bot - Requested by {author}";
        author_message = "fork of a fork, slightly altered bot by @baxter.zip, fork of vgskye's fork of valentinegbs goober bot - Requested by {author}";
        messages = [
            "fork of a fork, slightly altered bot by @baxter.zip, fork of vgskye's fork of valentinegbs goober bot - Requested by {author} for {user} for, some reason",
        ];
    }
}
silly_command! {
    /// Cuddle!!
    #[early_access]
    fn cuddle("Snuggly Cuddly Snuggles!!") {
        bot_message = "awwww, {author}, you're cuddling me?? thanksss~~~ <:floofOwO:1530411511481110548>";
        author_message = "Uh, {author} is just... cuddling themselves... I guess we should just let them be <:floofTired:1530411654976901250>";
        messages = [
            "{user} just got absolutely loved and cuddled by {author} <:floofHappy:1530411341180047400>",
            "{author}, just snagged up {user}, and now {user} got cuddled into oblivion <:floofMischief:1530411443587911680>",
            "Whoa, it turns out {author} was a secret cuddler the whole time- and now they're cuddling {user} <:floofOwO:1530411511481110548>",
            "{author} picked up {user} and gave big fluffy cuddles <:floofHappy:1530411341180047400>",
            "{author} gave {user} a life of free cuddles... dang, that's cute <:floofBlep:1530411290340753408>",
            "Hey there, uh, {user}? Is it? Yeah, uhm, we've got a cuddler named {author} for you... we're gonna have to put you under their infinite cuddles <:floofSmug:1530411628431147188>",
            "{author} is cuddling {user}? That's really cute :3 <:floofHappy:1530411341180047400>",
        ];
    }
}

silly_command! {
    /// whos it gonna be?
    #[early_access]
    fn makeout("Kiss²") {
        bot_message = " {author} THIS WAS NOT PART OF THE SCRIPT!! <:floofBlush:1530766136797429840>";
        author_message = "Uh, {author} is just... making out with themselves... how the fuck... im not gonna worry about it... <:floofTired:1530411654976901250>";
        messages = [
            "{user} just got absolutely smooched by {author}, <:floofNervous:1530411474776887366> wait why is there so much saliva <:floofNervous:1530411474776887366>",
            "{author}, just pulled {user} up to them, and now {user} and {author} are making out forever <:floofMischief:1530411443587911680>",
            "Whoa, {author} was a big kisser the whole time~ and now they're making out with {user} <:floofOwO:1530411511481110548>",
            "{author} picked up {user} and gave big... gross... kisses <:floofHappy:1530411341180047400>",
            "{author} graciously bestowed upon {user} a lifetime supply of big fat smoochy make outs... dang, that's intimate <:floofBlep:1530411290340753408>",
            "Hey there, uh, {user}? Is it? Yeah, uhm, we've got a person named {author} for you... we're gonna have to force you to make out with tme until the end of time <:floofSmug:1530411628431147188>",
            "{author} is making out with {user}? That's really cute :3 <:floofHappy:1530411341180047400>",
        ];
    }
}

silly_command! {
    /// :3
    fn cheekkiss("cuties") {
        bot_message = "\\*Awhhhhh {author} kissed me on the cheek!!! <:floofWoozy:1530411680419283014>";
        author_message = "{author} kissed themselves on the cheek? ...how? <:floofWhat:1530411667463340144>";
        messages = [
            "Hehehe- {author} gave {user} a peck on the cheek!!!! <:floofPlead:1530411552975360231>",
            "{author} gave {user} a little cheek smooch <:floofHappy:1530411341180047400>",
            "Hey guys, {author} gave {user} user a little cheek smooch!!! <:floofPlead:1530411552975360231>",
            "Hehehe {author} is so cute, they just kissed {user} on the cheek <:floofHappy:1530411341180047400>",
            "{author} not so violently pulled {user} to them and pekced them on the cheek, letting everyone else in <:floofHappy:1530411341180047400>",
        ];
    }
}

