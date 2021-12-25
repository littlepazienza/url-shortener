pipeline {
  agent none
  stages {
    stage('checkout') {
      agent {
        docker {
          image 'rust:latest'
          args '--volumes-from jenkins'
        }
      }
      steps {
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
            export JENKINS_NODE_COOKIE=/usr/local/homebrew/var/www/html/url.ienza.tech/short_url
            git pull --tags
            version=$(git describe)
            sed -i '' -e "s/<!--build_number-->/${version}/g" $WORKSPACE/www/index.html
            if [ $(ps -au$USER | grep short_url | wc -l) -eq 1 ]; then
              pkill -f short_url
            fi
            cp -r $WORKSPACE/www/* /usr/local/homebrew/var/www/html/url.ienza.tech
            cp -r $WORKSPACE/target/debug/* /usr/local/homebrew/var/www/html/url.ienza.tech
            nohup /usr/local/homebrew/var/www/html/url.ienza.tech/short_url &
          fi
        '''
      }
    }
  }
}
