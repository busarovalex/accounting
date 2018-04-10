pipeline {
    agent {
        master
    }
    stages {
        stage('Build') { 
            steps {
                sh 'cargo build' 
            }
        }
    }
}
