/backend
│
├── Cargo.toml                # Cargo manifest file for dependencies
├── src/
│   ├── main.rs               # Entry point for the Actix web server
│   ├── config.rs             # Application configuration (e.g., database, API keys)
│   ├── routes/
│   │   ├── mod.rs            # Handles routing for different endpoints
│   │   ├── rider.rs          # Handles rider-related requests (e.g., ride requests)
│   │   ├── driver.rs         # Handles driver-related requests (e.g., notifications, ride acceptance)
│   │   ├── tracking.rs       # Handles ride tracking and updates
│   │   ├── feedback.rs       # Handles rating and feedback submission
│   ├── services/
│   │   ├── mod.rs            # Service layer to interact with business logic
│   │   ├── ride_service.rs   # Handles ride creation, matching, and completion logic
│   │   ├── notification_service.rs  # Manages driver/rider notifications
│   │   ├── location_service.rs      # Handles location-based queries (e.g., driver proximity)
│   │   ├── feedback_service.rs      # Manages rating and feedback submissions
│   ├── models/
│   │   ├── mod.rs            # Structs and data models for the application
│   │   ├── rider.rs          # Rider model
│   │   ├── driver.rs         # Driver model
│   │   ├── ride.rs           # Ride model (status, time, etc.)
│   │   ├── feedback.rs       # Feedback model for storing ratings
│   ├── db/
│   │   ├── mod.rs            # Database connection and interactions
│   │   ├── ride_db.rs        # Ride-related database interactions
│   │   ├── feedback_db.rs    # Feedback-related database interactions
│   ├── utils/
│       ├── mod.rs            # Utility functions (e.g., location calculations, error handling)
│       ├── geolocation.rs    # Utility functions to calculate distances
│
├── migrations/               # Database migrations (if using SQL database)
│   └── ...
└── .env                      # Environment variables (e.g., database URL, API keys)
