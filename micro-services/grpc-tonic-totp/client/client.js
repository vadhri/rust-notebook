const {ValidateTotpPromiseClient}  = require('./user_totp_grpc_web_pb.js');
const {User, Totp, Register} = require('./user_totp_pb.js');

const grpc = {};
grpc.web = require('grpc-web');

console.log("ValidateTotpPromiseClient = ", ValidateTotpPromiseClient);

window.register_and_identify_service = new ValidateTotpPromiseClient('http://localhost:8081', null, null);

window.register = {};
window.register.create_user = async function create_user(fname, lname, email, callback) {
  const new_user = new User();

  new_user.setFirstname(fname);
  new_user.setLastname(lname);
  new_user.setEmail(email);

  let result = await window.register_and_identify_service.register(new_user, null);

  console.log("QR code =", result.getQrCode(), "Result = ", result.getRes(), "Token = ", result.getToken());
  window.register.token = result.getToken();
  callback({
    qrcode: result.getQrCode(),
    res: result.getRes(),
    token: result.getToken()
  });
}

window.register.validate_totp = async function create_user(code, token, callback) {
  const vaildation_code = new Totp();

  vaildation_code.setToken(token);
  vaildation_code.setInput(code);

  let result = await window.register_and_identify_service.validate(vaildation_code, null);

  console.log("Validation =", result.getRes());
  callback({
    res: result.getRes()
  });
}
