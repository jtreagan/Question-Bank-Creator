# Questions and Answers

Below are questions that other contributors have asked about QBC.

* **Why Question Bank Creator?**

My initial motivation for the project was to build a tool that I can personally use while writing an Algebra 1 curriculum.  There's a  lot of background to just why I want to write my own Algebra curriculum, but as far as QBC goes, I started writing it because I need it.  But  something like this, written open source and free to use, is just what  many teachers -- homeschool, private/church, or public -- need.  Other curriculum developers will find it useful.  The plan is for it to be something that can be used without any strings attached to some proprietary service.  

   * **I  notice we’re assuming that variables keep their positions when values change.**

Yes, that's a basic assumption for how the program is structured and implemented. Honestly, I don't have any idea how one would give the variables dynamic position or what context would require that. Something for the future, I suppose.

* **On the input side, are we aiming to make question/variable population as easy as possible for users?**

Yes. Of course.  That was why I moved to using the GUI.   That side of the application needs lots and lots of refinement, but making it easy to use is really important.

* **Is the long-term vision to let users define some abstract logic for solving a problem, and then have the system  apply that logic automatically to new variants?**

The application will be of little use  to a teacher or a student if it can't provide correct answers for  dynamically created questions.  At the time of this writing, that side of QBC has not yet been implemented, but it is imperative to get it working.  Basically, the user will need to solve the problem/equation in terms of the variables.  QBC will then use the values contained in each variable's `content` field to calculate the answer.  

**Here's an example**:  

Consider the equation 

​					`5x + 2 = 9`

The user might replace the `5` with a Variable named `coeff1`, the `2` with a Variable named `const1`, and the  `9` with a variable named `const2`.  So in the question the equation would look like:

​					`coeff1`x + `const1` = `const2`

When the user enters the answer to the question currently being created, he/she would first solve the above equation, entering it like this:

​					answer = (`const2` - `const1`) /  `coeff1`

QBC would then take that solved rendering of the equation, replace each Variable with the value contained in each respective `content` field, and calculate the answer.

I've noticed that there are some crates in crates.io that specialize in solving/evaluating equations.  Those crates might come in handy when we start implementing this part of the program.

* **You mentioned proprietary applications in this space — could you name a few?**

The one I used a lot while teaching was ***Exam View\***.  It was owned by Turning Technologies (which I see has merged with Echo 360).  At the time, I had purchased a copy of my own that I could use in my classroom.  I authored some question banks with the idea to offer them to other teachers through [teacherspayteachers.com](https://www.teacherspayteachers.com/), but Turning nixed it, claiming that I would be violating their copyright.  Like I  said, proprietary.  Exam View is a good app, but I can't use it for my  own projects because of that proprietary thinking on the part of the app's owners.

I did some adjunct teaching a couple of times.  At BMCC  (community college in Pendleton, OR) I used my copy of Exam View to  create materials for the classes I was teaching.  At Montana Tech University I used a textbook that the publisher Pearson had put together.  Their product let me have my students do their homework online using a tool similar to Exam View.  That was useful because it allowed my students to get immediate feedback as to whether they were doing the practice correctly.  (Of course, math can't be learned without plenty of practice.)  I  used their product to produce quizzes and tests in multiple  versions.  If a student was struggling, not passing a particular test/quiz, but was sincerely attempting the homework and trying to get help, I could have him/her retake the test/quiz multiple times using multiple versions until I was certain that they truly understood the topics being covered.  However, again, Pearson's tool is proprietary.

* **Do you envision machine learning (ML) ever playing a role here?**

I have never considered ML a part of QBC, but now that you bring it up,  it's certainly a possibility, perhaps even a probability, and maybe deserves serious attention.  Certainly not anytime soon, but maybe in the far, far future when much  better coders than me are helping with the project?  I have considered  the possible scenario of handing QBC over to an AI for it to use to parse a text, preparing it (the text) for teachers and students  to use, but that's not ML.  So ML?  Maybe, but I don't really  know what that will look like.

