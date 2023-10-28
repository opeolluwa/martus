# Sign up 
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{"email":"example@mailer.com", "password":"ddddd"}' 0.0.0.0:5001 martus_auth.Auth/SignUp