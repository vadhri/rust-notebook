module.exports = { createTimestampedObject };

function createTimestampedObject(userContext, events, done) {
  const data = { timestamp: Date.now(), hello: "world" };
  // set the "data" variable for the virtual user to use in the subsequent action
  userContext.vars.data = data;
  return done();
}
