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
        sh "rustup toolchain install nightly-2022-04-18"
        sh "rustup run nightly-2022-04-18 cargo -V"
        sh "rustup run nightly-2022-04-18 cargo test"
        sh "rustup run nightly-2022-04-18 cargo build"
      }
    }
    stage('package') {
      steps {
        sh '''
          if [ $GIT_BRANCH = "main" ]; then
            git pull --tags
            version=$(git describe --tags)
            sed -i -e "s/<!--build_number-->/${version}/g" $WORKSPACE/www/index.html
            cp -r $WORKSPACE/www/* /var/www/html/url.ienza.tech
          fi
          tar -czvf url-shortener.tar.gz *.toml src README.md
        '''
        archiveArtifacts artifacts: '*.tar.gz,**/*.html',
                   allowEmptyArchive: false,
                   fingerprint: true,
                   onlyIfSuccessful: true
      }
    }
  }
}
