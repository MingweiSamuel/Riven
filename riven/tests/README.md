Tests are divided up by region. This is because tests cannot share state, and therefore
cannot share the rate limiting done within a `RiotApi` instance. However, rate limiting
is separate for each region, so it is safe to send requests to different regions from
different instances.

The tests within an individual file do share their `RiotApi` instance thanks to custom
test runners and some macros I hacked together which are located in `async_tests.rs`.
They are set up in a way to look like normal test output for fun and probably to
confuse people.
