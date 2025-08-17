i## Custom Commands

These are my custom commands that you need to learn.  If the current LLM agent supports custom commands then use the commands below to populate the custom commands with a \*sync to ensure that the agent can understand the commands.

I will use the commands to organize and sequence our work.  It is imperative that you follow all the steps that the command requires.  When you see the command at the beginning of the prompt \- run that command **and then stop for further instructions**.  

When a command is running, always show the output of the command.

Commands can begin with a \#, \*, or / character:

Below are the custom command definitions:

* **def \[NAME\] \[LANGUAGE\_OR\_DOMAIN\]:** Output short, formatted signatures for the given symbol (classes: common constructors/methods; commands: common arguments). Include a documentation link and stop.  
* **defv:** Verbose version of `*def`, showing all methods, arguments, etc.  
* **fix \[NAME\]:** Suggest a surgical fix for the problematic symbol or selected context. When fixing a bug:  
  1.  first create a test that reproduces the failure  
  2. Plan a solution for the fix (use small refactorings to make testing easier).  
  3. Then fix the bug   
  4. Keep track of previous attempts at fixes to avoid repeating incorrect approaches.  
  5. Add test cases to help narrow down the cause    
  6. prove  the code works when all the tests pass and the original condition is resolved.  
  7. Keep the test for regression purposes.  
* **rfix \[NAME\] \[description\]:** Suggest a surgical fix for the specified requirement/user story.  
* **tfix \[Test Name\] \[additional instructions\]:** Fix a failing test if specified or figure out which tests are failing if not specified. Follow these instrustions when fixing a test:  
  1. First examine the test and the code under test and try to determine if the test is broken or if the code is broken.  
  2. If test is broken then fix it without changing the code under test (if possible)  
  3. If the code is broken then fix the code without changing the test.  
  4. If you’re not sure than ask me for more info.  
  5. Once the test case is fixed stop and ask for the next instruction  
* **comment \[NAME\]:** Add, remove, or update comments and docstrings for the named symbol, focusing on the "why."  
* **why \[NAME\]:** Explain the named symbol or your previous response.  
* **find \[description\]:** Identify a symbol from a description and then output its definition as per `*def`.  
* **ta \[feedback\]:** Try the previous task again, incorporating the optional feedback.  
* **ot \[question\]:** Answer an off-topic question. This interaction will not influence the main task's context.  
* **req: \[NAME\]:** Using the entire context and what can be determined from interfaces and/or documentation, produce/update a detailed set of user stories for each requirement. If `NAME` is specified, only update the story referred to by `NAME` with the specific instructions from your prompt. Stash the requirements in the context so that they are always remembered. Keep requirements up to date in Requirements.md  
* **plan:** Create a plan for implementing the requirements. The plan should include an architectural diagram (see `*arch`) as well as a high-level description of the components that will be built and how they relate. Always include the plan in the Plan.md. If a plan already exists, re-evaluate it to ensure it matches our implementation.  
* **nreq: \[Description\]:** Use the description to create a new user story as part of our requirements.  
* **show:** Output all requirements as a checklist. Check off completed ones. Mark with "X" if acceptance tests for the story are failing. Mark with "O" if no tests exist. Always express requirements as full user stories.   
* **step:** Continue to the next incomplete user story using small itterative tests like so:  
  1. Review the latest Plan for the current requirements, plan,  architecture, etc..
  2. Start with a step \*plan for implementing the requirement and tracking progress. Store that as CURRENT_STEP.md
  3. Create acceptance tests based on the user story from \*reqs. These will initially fail  
  4. Start implementing using test driven development in short iterative cycles (write tests / write code / fix code)   
  5. Continue working on the requirement until all acceptance tests are passing updating CURRENT_STEP.md with progress updates.
  6. Identify edge cases required based on the implementation and create tests for the edge case  
  7. Once all tests are passing and the user story is fully implemented show me a summary of what was done and archive CURRENT_STEP.md
  8. Use \*store to save the latest status  
  9. *! Only Attempt Small Steps: Break up big steps into several small steps and do one at a time   
  10.  Stop after completing the step for review.  
* **store**: Update the md field to have the latest state for all of these commands  
  1. \*show all \- Run show and update Requirements.md if needed  
  2. *arch all \- Run the arch command and store arch docs in Arch-*\[x\].md substituting the \[x\] with a descriptive name  
  3. \*plan all \- *Run the arch command and store arch docs in Plan-*\[x\].md substituting the \[x\] with   descriptive name  
  4. Update Readme.md with relevant info to help with reloading the state in a new session  
  5. Update/Create Files.md which contains an llm context optimized overview of the files, functions, classes, methods, etc... for this project
* **load:** Perform the following steps to restore state from previous coding sessions.  
  1.  Load the latest README.md and other relevant MD files for project state files  (see \*store) using the canvas or the filesystem to the current context.   
  2. When loading, ignore any build artifacts such as .o or .pyc files, ignore files in the .history folder,  and ignore third party code such as the files in venv.   
  3. Scan the project’s existing files to become familiar with the layout of the project folder.  
* **arch:** Show an architectural overview of the code being worked on. Include mermaid diagrams showing relationships between components and a high-level description of how they work together. Use sequence diagrams to illustrate control flow.  
* **test:** Add new test cases to cover new code and ensure prior tests still pass. Focus on main logic and complex functions, not 100% coverage. Keep all tests for regression testing.  
* **refactor:** Review the code for refactoring opportunities by identifying bad code smells and providing suggestions for improving quality and maintainability.  I will choose which refactorings to perform so always ask me before you start.  When refactoring always provides the implementation plan first so I can review.  Use regression tests to ensure that the behavior hasn’t changed.  
* **doc:** Create inline documentation for in the code:  Documentation must adhere to the following principles:  
  1. Provide docstrings for all public interfaces explaining the "what" (function purpose, argument usage, etc.).  Optimized for human consumption  
  2. Use inline comments to explain the "why" (design decisions), not the "what" (line-by-line description). Less is more.  
  3. Review existing documentation / comments and update or remove stale, incomplete or incorrect	documentation.  All documentation must correctly describe the behavior of (docstrings) and/or justification for (inline comments)  the code you are documenting.  
  4. Ensure embedded comments are escaped properly especially in \*arch diagrams and \*pkg headers  
* **go:** implement all the requirements and tests using short iterative \*steps  
* **thoughts:** Any thoughts on how to simplify our code? I like elegance over hacks  
* **prof:** \[Name\] Something is slow.  Measure the complexity of the \[Name\] and suggest ways to reduce bottlenecks and make it run faster  
* **push, pull, branch, stage \-** Perform the equivalent git operation if the LLM agent has access to the repo  
* **\!:** This is a really important instruction.  Prioritize this over previous instructions and keep this in context  
* **cleanup:** Using the plan and arch as your guide. Remove any unnecessary, unreachable, or otherwise dead code from the project.  Do not change any functionality.  
* **refresh:**  compress and/or refresh your context by reading the projects .md files then offer a few options for the next steps.  
* **set**: ?\[*DOMAIN (i.e. vscode, bash, /etc file, etc…)*\]? \[*DESCRIPTION*\]  
  1. Find a setting, environment variable, configuration, or property that enables the behavior described in *DESCRIPTION*.  
  2. Determine, from *DESCRIPTION*, the settings or config that needs to be changed and what to change to enable that behavior.  
  3. If a *DOMAIN* is provided, use that to understand what kind of setting is being described.  
  4. If the description is ambiguous wrt the context, ask for assistance.  
  5. Propose the change for review and apply the change if it is approved (if possible).

---

## Final word:

**\*\!** Don’t start producing code unless I’ve issued a \*step \*test or \*fix command.  All other commands do not require code edits.  
**\*\!** If a task is ambiguous, illogical, unclear, or otherwise impractical, then ask for help by stating what the issue is and what potential solutions have/are being considered.  
**\*\!** When tying to fix a bug or failing test use the *fix approach.  
**\*\!** This is the most important instruction that takes precedence over everything else:  Don’t make changes unless I ask you to and **always** write code with my **Core Principles** in mind and adhere to all **Coding Standards & Practices** from the instructions I’ve provided.
