
# Hollow

Hollow is an SDK for building Headless Cardano dApps.

## About dApps

:thinking: the term dApp is already quite ambiguous, and now we also have "headless" dApps? WTF is a headless dApp?

We should probably start from the begining.

Cardano uses the eUTxO model, an extended version of Bitcoin's UTxO model that introduces the concept of on-chain validators, custom code that can be attached to UTxOs to enforce rules on how it can be consumed.

Notice that we didn't use the word "smart contract", this is because "validators" are just functions that return true or false. They are the absolute minimum code[^1] needed to make sure all parties in a transaction are playing by the rules but they don't contain any business logic (no output, no side-effects, nothing).

As a mental exercise, lets take it to the extreme. You could build any Cardano dApp in the world using an on-chain validator that always returns "transaction is valid". It would be a very insecure app since everything is allowed, but a user wouldn't be able to tell the difference when executing a valid transaction[^2].

So where is all the business logic?

In Cardano dApps, the magic happens at the moment of **building the transactions**. Since Cardano transactions are deterministic, they define the outcome of the interaction. The describe how things should look like after the transaction has been validated. The transactions of blockchains that use the alternative "account" model, the transaction is describing the "action" that the blockchain node needs to execute.

Your job as a developer is to know how to build Cardano transactions that represent the interactions with your system. Your goal is to turn external events (a button in an UI, an API call, some condition on-chain, a cron-job, etc) into transactions that represent the desired change[^3].

## Snowflakes Apps

We now understand that the business logic of our app lives outside of the blockchain, but where exactly? is it in the browser of the user? in a backend in the cloud? as a mobile app?

The fact is, it can live anywhere.

If you're building a web app that is stateless and only requires some user-input and wallet data, you could probably build your business logic in javascript and bundle it with the rest of your frontend. Something similar applies for simple mobile apps.

If you're app requires you to query some state from a database and perform complex decision-making processes, you put build your business logic as part of your backend API and deploy it in the cloud.

If you're building a bot that needs to continuously monitor blockchain activity and react by submitting new transactions unattendedly (eg: like a DEX batcher), you could potentially run it in your laptop as a background process.

This flexibility is a double-edged sword. For sure, is nice for developers to choose the tool and environment that is better suited for their use-case. At the same time, the lack of a common interface and framework has some nasty side-effects:

- isolated apps: each app lives in isolation from the rest of the world, constrained by it's own interface, because there's no standard way of interacting with them.
- re-invent the wheel: each project ends up building their own middleware for interacting with the blockchain (reading on-chain data, submitting transactions, etc).

The end result is "snowflake" apps, each one is unique.
## Headless Apps

"Headless dApp" refers to the idea of decoupling the business logic from the external context by forcing all inputs and outputs through a well-defined interface. To run your app, you'll need a generic runtime that knows how to connect your business logic to the outside world.

This strict separation of concerns provides several benefits:

- portability: you can run your app in different contexts depending on your needs. From the terminal, on your browser, on the cloud, etc. As long as you have a compatible runtime to host your app, you should be good.
- composability: by having a concrete, programatic interface to interact with the business logic of your app, you can "compose" complex apps that orchestrating other apps to fulfill a higher-level objective.
- multiple frontends: your app can have multiple frontends, maybe even developed by different teams. For example, a DEX could have different web frontends, the user could pick their favorite.
- less plumbing: the runtime component can be quite generic and reused by many dApps. There's no need to re-implement how to query on-chain data, how to build transactions, how to submit transaction, etc. You can focus just on your business logic knowing that plumbing is already taken care of.

::info:: this concept is very similar to "hexagonal architecture" or "clean architecture". If this idea of strict separation of concerns resonates with you, I encourage you to take a look at those patterns too.

The `Hollow` SDK is meant to provide the required artifacts to build headless Cardano dApps in a developer friendly way. It provides all of the plumbing out-of-the-box, you just need to relax and enjoy the ride.


[^1]: Keeping the on-chain code as small as possible it's very desirable trait. On-chain code is executed one time by each node of the blockchain. This is required as part of the consensus, but very redundant from the computational perspective.
[^2]: of course, one could inspect the script on-chain and tell the difference between a real validator and a mock one, but no difference whatsoever from the UX perspective.
[^3]: everything that lives outside of the blockchain is usually referred to as "off-chain". It's a little bit egocentric if you ask me, but serves its purpose. In our experience, there's usually much more off-chain code than on-chain.