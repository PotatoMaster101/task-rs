/**
* Handler that will be called during the execution of a PostLogin flow.
* This handler adds or assigns roles. Roles will be added on user register, and assigned on user login.
*
* @param {Event} event - Details about the user and the context in which they are logging in.
* @param {PostLoginAPI} api - Interface whose methods can be used to change the behavior of the login.
*/
exports.onExecutePostLogin = async (event, api) => {
  if (!event.authorization) {
    return
  }

  const namespace = 'https://task'
  const key = `${namespace}/roles`
  if (event.authorization.roles?.length > 0) {
    // existing user logging in
    api.idToken.setCustomClaim(key, event.authorization.roles)
    api.accessToken.setCustomClaim(key, event.authorization.roles)
    return
  }

  // new user signed up
  const ManagementClient = require('auth0').ManagementClient
  const management = new ManagementClient({
    // set secrets from API app
    clientId: event.secrets.CLIENT_ID,
    clientSecret: event.secrets.CLIENT_SECRET,
    domain: event.secrets.DOMAIN,
  })

  const params = { 'id': event.user.user_id }
  const data = { 'roles': [event.secrets.DEFAULT_ROLE_ID] }
  try {
    await management.users.assignRoles(params, data)
    api.idToken.setCustomClaim(key, [event.secrets.DEFAULT_ROLE_NAME])
    api.accessToken.setCustomClaim(key, [event.secrets.DEFAULT_ROLE_NAME])
  } catch (ex) {
    console.error(ex)
  }
}
