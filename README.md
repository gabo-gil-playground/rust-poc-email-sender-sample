# rust-poc-email-sender-sample

POC email sender sample is a SMTP/TLS API IntegrationA high-performance Proof of Concept (POC) demonstrating a secure bridge between a RESTful API and SMTP infrastructure.

This project serves as a foundation for building scalable notification services using Gmail's SMTP server with mandatory TLS encryption.

## 🚀 Overview
This POC is designed to handle transactional email delivery through a clean, decoupled architecture. It focuses on security best practices, ensuring that sensitive credentials never touch the codebase by utilizing environment-driven configuration.
### Key Technical FeaturesProtocol:
SMTP over TLS (Transport Layer Security) for encrypted transmission.
### Infrastructure:
Configured for Gmail SMTP relay.
### Security:
Zero-leak credential management via Environment Variables.

### API Design:
Stateless JSON-based endpoint for seamless integration with frontend or microservices.

## 🛠️ Configuration
The application requires the following environment variables to be set. This ensures the POC follows the Twelve-Factor App methodology for configuration.
### VariableDescription
| Variable | Description | Example |
| :--- | :--- | :--- |
| `APP_EMAIL_ADDRESS` | The Gmail account used to send emails. | `your-service@gmail.com` |
| `APP_EMAIL_PASSWORD` | An [App Password](https://myaccount.google.com/apppasswords) generated via Google. | `xxxx xxxx xxxx xxxx` |

#### Security Note: Do not use your primary Google password. Always generate a dedicated App Password with 2FA enabled.

## 🏗️ Architecture & FlowRequest:
The API receives a structured JSON payload via the /send endpoint.

### Authentication:
The service authenticates with smtp.gmail.com using TLS on port 587.
### Handshake:
A secure STARTTLS command is issued to encrypt the communication channel.
### Delivery:
The email is dispatched to the targetAddress.

## 💻 Usage
```bash
# 1. Installation
# Clone the repository
git clone [https://github.com/gabo-gil-playground/rust-poc-email-sender-sample](https://github.com/gabo-gil-playground/rust-poc-email-sender-sample)

# Install dependencies (Example for Java/Rust)
cargo build

#2. Export your credentials
export APP_EMAIL_ADDRESS="your-email@gmail.com"
export APP_EMAIL_PASSWORD="your-app-password"

# 3. Start the service (runs on port 9026 by default)
* use your IDE run options
* run cargo from command line

# 4. API DocumentationThe service exposes a single versioned endpoint for email dispatch.
# Endpoint: GET /api/v1/email/send
curl --request GET \
  --url http://localhost:9026/api/v1/email/send \
  --header 'Content-Type: application/json' \
  --data '{
	"targetAddress": "some.email.address@gmail.com",
	"emailSubject": "some subject",
	"emailBody": "Some text"
}'
```   
## 🛡️ Production Roadmap
To transition this POC into a production-ready microservice, the following modules are recommended:
* [ ] Queueing: Integration with RabbitMQ or Redis for asynchronous processing.
* [ ] Observability: Prometheus metrics for delivery success/failure rates.
* [ ] Rate Limiting: Protect the SMTP relay from abuse.
* [ ] Templates: Support for Handlebars or MJML for rich HTML emails.

## Author
Gabo Gil – Senior Software Architect
* Specialized in High-Performance Backend Systems & Distributed Architecture.
* https://www.gabogil.com/

## Contact
Blog: https://somedevnotes.wordpress.com/

Github: https://github.com/gabo-gil-playground

Linkedin: https://www.linkedin.com/in/gabogil/

