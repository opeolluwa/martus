# health check
rpcurl --vv --plaintext -proto ./proto/auth.proto -d '{}' 0.0.0.0:5001 martus_auth.Auth/HealthCheck


# Sign up 
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{"email":"adefemiadeoye@yahoo.com", "password":"ddddd"}' 0.0.0.0:5001 martus_auth.Auth/SignUp

# login 
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{"email":"examhssnnple@mailer.com", "password":"ddddd"}' 0.0.0.0:5001 martus_auth.Auth/Login

#logout 
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{"token":"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJjbGFpbSI6eyJpZCI6IjA3NGVjNzcxLTlmOGQtNGU2OS1iYzIxLTJhOGY0NzZhZjgxNSIsImVtYWlsIjoiZXhhbWhzc25ucGxlQG1haWxlci5jb20ifX0.IUyzrEgLTGbhmUoAv-ZwoCiPTjbbyR4CWVGyy5voniI97wgJIREty6ShX6mvzMWg80PvtbxApfuZXyUQc7Q65g"}' 0.0.0.0:5001 martus_auth.Auth/Logout


# see queue content 
sudo  /usr/local/kafkabin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic "email-queue" --from-beginning
