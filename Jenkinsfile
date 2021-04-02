pipeline {
  agent {
    docker {
      image 'rust:latest'
      args '-v /var/www/html/url.ienza.tech/:/root/url.ienza.tech'
    }
  }
  stages {
    stage('checkout') {
      steps {
        checkout scm
        sh "rustup default nightly"
      }
    }
    stage('test') {
      steps {
        sh "cargo test"
      }
    }
    stage('build') {
      steps {
        sh "cargo build"
      }
    }
    stage('deploy') {
      steps {
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            git pull --tags
            git describe >> ./target/debug/version.txt
          fi
        '''
      }
    }
  }
  post {
    success {
      sh '''
        ls -lr $WORKSPACE
        pkill -f short_url
        nohup /var/www/html/url.ienza.tech/short_url &
      '''
    }
  }
}
