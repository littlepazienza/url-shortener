pipeline {
  agent none
  stages {
    stage('checkout') {
      agent {
        docker {
          image 'rust:latest'
        }
      }
      steps {
        checkout scm
        sh "rustup default nightly"
        sh "cargo test"
        sh "cargo build"
      }
    }
    stage('deploy') {
      agent any
      steps {
        JENKINS_NODE_COOKIE=dontKillMe /var/www/html/url.ienza.tech/short_url
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            git pull --tags
            git describe >> $WORKSPACE/target/debug/version.txt
            cp -r $WORKSPACE/target/debug/* /var/www/html/url.ienza.tech
            nohup /var/www/html/url.ienza.tech/short_url &
          fi
        '''
      }
    }
  }
}
