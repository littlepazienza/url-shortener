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
      agent any
      steps {
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            git pull --tags
            git describe >> $WORKSPACE/target/debug/version.txt
            cp $WORKSPACE/target/debug/* /var/www/html/url.ienza.tech
            pkill -f short_url
            nohup /var/www/html/url.ienza.tech/short_url &
          fi
        '''
      }
    }
  }
}
