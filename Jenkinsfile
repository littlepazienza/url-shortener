pipeline {
  agent {
    docker {
      image 'rust:latest'
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
            echo $HOME
            git describe >> ./target/debug/version.txt
          fi
        '''
      }
    }
  }
  post {
    success {
      sh '''
        pkill -f short_url
        nohup /var/www/html/url.ienza.tech/short_url &
      '''
    }
  }
}
