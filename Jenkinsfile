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
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            export JENKINS_NODE_COOKIE=/var/www/html/url.ienza.tech/short_url
            git pull --tags
            git describe >> $WORKSPACE/target/debug/version.txt
            if [ $(ps -aujenkins | grep short_url | wc -l) -eq 1 ]; then
              pkill -f short_url
            fi
            cp -r $WORKSPACE/target/debug/* /var/www/html/url.ienza.tech
            nohup /var/www/html/url.ienza.tech/short_url &
          fi
        '''
      }
    }
  }
}
