# Sign up 
grpcurl --vv --plaintext -proto ./proto/auth.proto -d '{"email":"examhple@mailer.com", "password":"ddddd"}' 0.0.0.0:5001 martus_auth.Auth/SignUp

# see queue content 
sudo bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic "email-queue" --from-beginning
