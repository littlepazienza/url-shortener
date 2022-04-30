pipeline {
  agent any
  stages {
    stage('build') {
      agent {
          docker {
              image 'rust:latest'
          }
      }
      steps {
        sh "rustup default nightly"
        sh "cargo test"
        sh "cargo build"
      }
    }
    stage('package') {
      steps {
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            git pull --tags
            version=$(git describe --tags)
            sed -e "s/<!--build_number-->/${version}/g" $WORKSPACE/www/index.html
            cp -r $WORKSPACE/www/* /var/www/html/url.ienza.tech
          fi
          zip -r url-shortener.zip *.toml src README.md
        '''
        archiveArtifacts artifacts: '*.zip,**/*.html',
                   allowEmptyArchive: false,
                   fingerprint: true,
                   onlyIfSuccessful: true
      }
    }
  }
}
