pipeline {
  agent {
    docker {
      image 'rust:latest'
    }
  }

  stages {

    stage('Build') {
      steps {
        sh "cargo build"
      }
    }
    stage('Test') {
	sh "cargo test"
    }
  }
}
