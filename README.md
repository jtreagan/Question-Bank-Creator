# Question Bank Creator

For use by homeschool parents and Christian/public/private school teachers.

## A Word of Warning

I'm an amateur programmer and this code is rough.  Please don't let that rough code put you off.  Right now the goal is to get an initial prototype going.  I've left a lot of stuff undone, the idea being that those things can be dealt with in the next iteration of the code.  This project is nowhere close to completion, so keep that in mind as you play around with it.

## Description

This program is targeted at teachers & homeschool parents and is especially useful for teaching math and science, although it also can be useful as an aide in the teaching of other disciplines.  Curriculum developers will especially find it useful.  It allows the construction of test/worksheet/quiz/individual practice questions that contain dynamic content.  It then saves those questions -- using the json markdown language -- in user-defined ‘question banks’, thus keeping related questions together in the same file.  A parent or teacher can create variables that generate dynamic values (either numeric, character, or string) using random or pseudo-random criteria set by the user.  Once constructed, the question is stored in a file (or 'question bank') for later access as needed.  Teachers can also make the app/questions available to students for student-directed practice.

## More detailed description and example

Here’s an example:  Suppose you want to teach your student how to solve a simple distance-rate-time problem and the text of the problem looks something like this:

   * Coach Roberts told Jimmy that he ran the 10 km race at last Saturday’s track meet in 70 minutes.  What was his average speed?

In the above problem the numbers ‘10’ and ‘70’ would each be replaced by a dynamic variable that randomly generates values from within teacher-proscribed boundaries.  For instance the “distance” variable might allow for integer values between 8 and 15 with the teacher setting those boundaries (presumably so that the problem remains plausible).  The “time” variable might run from 60 to 120 in one decimal place increments, again with the teacher setting the variable’s limits.  When the problem is outputted for use in a worksheet or quiz, the variables are recalculated.  For instance, the recalculated problem might look like:

   * Coach Roberts told Jimmy that he ran the 12 km race at last Saturday’s track meet in 90.3 minutes.  What was his average speed?

To make the problem more interesting, the teacher might have the two personal names in the problem be randomly chosen from lists.  Maybe the lists might look like this:

   * Coaches:  {Roberts, Jones, Gupta, Baldwin, Gaskill}
   * StudentNames:  {Jimmy, Andrea, Bob, Julia, Harry, Clem, Mabel}

Then the names in the problem could be replaced by variables that randomly choose values from those respective lists.  Now, a recalculated problem could look like this:

   * Coach Baldwin told Clem that he ran the 11 km race at last Saturday’s track meet in 85.7 minutes.  What was his average speed?

Thus the problem and the skills required to solve it remain essentially the same with the numbers (and answers) being different.  This allows the student to practice until he/she feels confident.  Of course, in a classroom situation, the teacher can also use this feature to print multiple versions of a test or quiz to prevent cheating.

## Dependencies

Note that this app is not yet usable.  The project is currently being developed in the Rust language and the Linux operating system.  It uses the following crates found in the Rust language's crates.io.
   
   * serde-json
   * fltk-rs
   * num-traits
   
It also uses several unpublished crates developed by the author.  Here is a copy of those dependencies from my local Cargo.toml file:

### Reagan created crates
   * lib_jt 
   * lib_file 
   * lib_input_fltk 
   * lib_lists 
   * lib_utils 
   * lib_myfltk

I am in the process of pushing these up to Github and will modify this file when I have it working.

## Author
   * John T. Reagan
   * johntreagan@gmail.com

## License

This project is licensed under the GNU General Public License v3.0 License.  See the LICENSE.md file for details

























