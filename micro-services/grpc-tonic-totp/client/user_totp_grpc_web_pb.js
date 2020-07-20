/**
 * @fileoverview gRPC-Web generated client stub for register_and_identify
 * @enhanceable
 * @public
 */

// GENERATED CODE -- DO NOT EDIT!


/* eslint-disable */
// @ts-nocheck



const grpc = {};
grpc.web = require('grpc-web');

const proto = {};
proto.register_and_identify = require('./user_totp_pb.js');

/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.register_and_identify.ValidateTotpClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @param {string} hostname
 * @param {?Object} credentials
 * @param {?Object} options
 * @constructor
 * @struct
 * @final
 */
proto.register_and_identify.ValidateTotpPromiseClient =
    function(hostname, credentials, options) {
  if (!options) options = {};
  options['format'] = 'text';

  /**
   * @private @const {!grpc.web.GrpcWebClientBase} The client
   */
  this.client_ = new grpc.web.GrpcWebClientBase(options);

  /**
   * @private @const {string} The hostname
   */
  this.hostname_ = hostname;

};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.register_and_identify.Totp,
 *   !proto.register_and_identify.AuthenticationResult>}
 */
const methodDescriptor_ValidateTotp_Validate = new grpc.web.MethodDescriptor(
  '/register_and_identify.ValidateTotp/Validate',
  grpc.web.MethodType.UNARY,
  proto.register_and_identify.Totp,
  proto.register_and_identify.AuthenticationResult,
  /**
   * @param {!proto.register_and_identify.Totp} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.register_and_identify.AuthenticationResult.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.register_and_identify.Totp,
 *   !proto.register_and_identify.AuthenticationResult>}
 */
const methodInfo_ValidateTotp_Validate = new grpc.web.AbstractClientBase.MethodInfo(
  proto.register_and_identify.AuthenticationResult,
  /**
   * @param {!proto.register_and_identify.Totp} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.register_and_identify.AuthenticationResult.deserializeBinary
);


/**
 * @param {!proto.register_and_identify.Totp} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.register_and_identify.AuthenticationResult)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.register_and_identify.AuthenticationResult>|undefined}
 *     The XHR Node Readable Stream
 */
proto.register_and_identify.ValidateTotpClient.prototype.validate =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/register_and_identify.ValidateTotp/Validate',
      request,
      metadata || {},
      methodDescriptor_ValidateTotp_Validate,
      callback);
};


/**
 * @param {!proto.register_and_identify.Totp} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.register_and_identify.AuthenticationResult>}
 *     A native promise that resolves to the response
 */
proto.register_and_identify.ValidateTotpPromiseClient.prototype.validate =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/register_and_identify.ValidateTotp/Validate',
      request,
      metadata || {},
      methodDescriptor_ValidateTotp_Validate);
};


/**
 * @const
 * @type {!grpc.web.MethodDescriptor<
 *   !proto.register_and_identify.User,
 *   !proto.register_and_identify.RegistrationResult>}
 */
const methodDescriptor_ValidateTotp_Register = new grpc.web.MethodDescriptor(
  '/register_and_identify.ValidateTotp/Register',
  grpc.web.MethodType.UNARY,
  proto.register_and_identify.User,
  proto.register_and_identify.RegistrationResult,
  /**
   * @param {!proto.register_and_identify.User} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.register_and_identify.RegistrationResult.deserializeBinary
);


/**
 * @const
 * @type {!grpc.web.AbstractClientBase.MethodInfo<
 *   !proto.register_and_identify.User,
 *   !proto.register_and_identify.RegistrationResult>}
 */
const methodInfo_ValidateTotp_Register = new grpc.web.AbstractClientBase.MethodInfo(
  proto.register_and_identify.RegistrationResult,
  /**
   * @param {!proto.register_and_identify.User} request
   * @return {!Uint8Array}
   */
  function(request) {
    return request.serializeBinary();
  },
  proto.register_and_identify.RegistrationResult.deserializeBinary
);


/**
 * @param {!proto.register_and_identify.User} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @param {function(?grpc.web.Error, ?proto.register_and_identify.RegistrationResult)}
 *     callback The callback function(error, response)
 * @return {!grpc.web.ClientReadableStream<!proto.register_and_identify.RegistrationResult>|undefined}
 *     The XHR Node Readable Stream
 */
proto.register_and_identify.ValidateTotpClient.prototype.register =
    function(request, metadata, callback) {
  return this.client_.rpcCall(this.hostname_ +
      '/register_and_identify.ValidateTotp/Register',
      request,
      metadata || {},
      methodDescriptor_ValidateTotp_Register,
      callback);
};


/**
 * @param {!proto.register_and_identify.User} request The
 *     request proto
 * @param {?Object<string, string>} metadata User defined
 *     call metadata
 * @return {!Promise<!proto.register_and_identify.RegistrationResult>}
 *     A native promise that resolves to the response
 */
proto.register_and_identify.ValidateTotpPromiseClient.prototype.register =
    function(request, metadata) {
  return this.client_.unaryCall(this.hostname_ +
      '/register_and_identify.ValidateTotp/Register',
      request,
      metadata || {},
      methodDescriptor_ValidateTotp_Register);
};


module.exports = proto.register_and_identify;

