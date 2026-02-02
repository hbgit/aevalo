pipeline {
    agent any

    environment {
        REGISTRY = 'docker.io'
        BACKEND_IMAGE = "${REGISTRY}/aevalo/backend:${BUILD_NUMBER}"
        FRONTEND_IMAGE = "${REGISTRY}/aevalo/frontend:${BUILD_NUMBER}"
        DOCKER_CREDENTIALS = credentials('docker-hub-credentials')
    }

    stages {
        stage('Checkout') {
            steps {
                echo '📦 Checking out code...'
                checkout scm
            }
        }

        stage('Build Backend') {
            steps {
                echo '🔨 Building Rust backend...'
                dir('backend') {
                    sh '''
                        cargo build --release
                        cargo test
                    '''
                }
            }
        }

        stage('Build Frontend') {
            steps {
                echo '🎨 Building Vue.js frontend...'
                dir('frontend') {
                    sh '''
                        npm ci
                        npm run build
                        npm run type-check
                    '''
                }
            }
        }

        stage('Lint & Code Quality') {
            parallel {
                stage('Backend Lint') {
                    steps {
                        echo '🧹 Linting Rust code...'
                        dir('backend') {
                        sh '''
                            cargo fmt -- --check
                            cargo clippy -- -D warnings
                        '''
                        }
                    }
                }
                stage('Frontend Lint') {
                    steps {
                        echo '🧹 Linting Vue code...'
                        dir('frontend') {
                        sh 'npm run lint'
                        }
                    }
                }
            }
        }

        stage('Build Docker Images') {
            steps {
                echo '🐳 Building Docker images...'
                sh '''
                    docker build -f infra/docker/Dockerfile.backend -t ${BACKEND_IMAGE} .
                    docker build -f infra/docker/Dockerfile.frontend -t ${FRONTEND_IMAGE} .
                '''
            }
        }

        stage('Push Docker Images') {
            when {
                branch 'main'
            }
            steps {
                echo '📤 Pushing Docker images to registry...'
                sh '''
                    echo ${DOCKER_CREDENTIALS_PSW} | docker login -u ${DOCKER_CREDENTIALS_USR} --password-stdin
                    docker push ${BACKEND_IMAGE}
                    docker push ${FRONTEND_IMAGE}
                    docker logout
                '''
            }
        }

        stage('Deploy to Staging') {
            when {
                branch 'main'
            }
            steps {
                echo '🚀 Deploying to staging environment...'
                sh '''
                    docker-compose -f docker-compose.yml down
                    docker-compose -f docker-compose.yml up -d
                '''
            }
        }

        stage('Health Check') {
            steps {
                echo '✅ Running health checks...'
                sh '''
                    sleep 10
                    curl -f http://localhost/health || exit 1
                    curl -f http://localhost:3000/health || exit 1
                    curl -f http://localhost:9090/-/healthy || exit 1
                '''
            }
        }

        stage('Integration Tests') {
            steps {
                echo '🧪 Running integration tests...'
                sh '''
                    # GraphQL endpoint health
                    curl -X POST http://localhost:3000/graphql \
                        -H "Content-Type: application/json" \
                        -d '{"query":"{ evaluations { id } }"}' || exit 1
                '''
            }
        }

        stage('Deploy to Production') {
            when {
                branch 'main'
                tag 'v*'
            }
            input {
                message "Deploy to production?"
                ok "Deploy"
            }
            steps {
                echo '🎯 Deploying to production...'
                sh '''
                    # Production deployment commands
                    # docker tag ${BACKEND_IMAGE} ${REGISTRY}/aevalo/backend:latest
                    # docker tag ${FRONTEND_IMAGE} ${REGISTRY}/aevalo/frontend:latest
                    # docker push ${REGISTRY}/aevalo/backend:latest
                    # docker push ${REGISTRY}/aevalo/frontend:latest
                    echo "Production deployment triggered"
                '''
            }
        }
    }

    post {
        always {
            echo '🧹 Cleaning up...'
            sh 'docker logout'
        }
        success {
            echo '✅ Pipeline completed successfully!'
        }
        failure {
            echo '❌ Pipeline failed!'
            // Add notification logic (Slack, email, etc.)
        }
    }
}
