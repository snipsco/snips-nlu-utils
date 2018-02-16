@Library('snips@0.0') _

build = [
    stableBranch: "master",
    devBranch: "develop",
]

def builders = [:]
def branchName = "${env.BRANCH_NAME}"

def version(path) {
    readFile("${path}/__version__").split("\n")[0]
}

def uploadAsset(pythonPath, venvName, asset="bdist_wheel") {
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"

    def VIRTUALENV = "virtualenv -p $pythonPath $venvName"
    def VENV = ". ${venvName}/bin/activate"

    sh """
    cd python
    ${VIRTUALENV}
    ${VENV}
    pip install -r requirements.txt
    ssh-agent sh -c \" ssh-add; python setup.py ${asset} upload -r pypisnips \"
    """
}

def buildAndTest(pythonPath, venvName) {
    def branchName = "${env.BRANCH_NAME}"
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"

    def VIRTUALENV = "virtualenv -p $pythonPath $venvName"
    def VENV = "source ${venvName}/bin/activate"

    stage('Setup') {
        deleteDir()
        checkout scm
        sh """
        cd python
        rm -rf ${venvName}
        ${VIRTUALENV}
        ${VENV}
        pip install -r requirements.txt
        """
    }

    stage('Rust build') {
        sh "cargo build --all"
    }

    stage('Rust tests') {
        sh "cargo test --all"
    }

    stage('Python build') {
        sh """
        cd python
        ${VENV}
        ssh-agent sh -c \" ssh-add; pip install -e . --verbose \"
        """
    }

    stage('Python tests') {
        sh """
        cd python
        ${VENV}
        python -m unittest discover
        """
    }
}


node('jenkins-slave-ec2') {

    stage('Python build') {
        builders['linux-x86_64'] = {
            node('jenkins-slave-ec2') {
                def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                def python34path = sh(returnStdout: true, script: 'which python3.4').trim()

                buildAndTest(python27path, "venv27")
                buildAndTest(python34path, "venv34")
            }
        }

        builders['apple-macos-compile'] = {
           node('macos') {
                def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                def python34path = sh(returnStdout: true, script: 'which python3.4').trim()
                def python35path = sh(returnStdout: true, script: 'which python3.5').trim()
                def python36path = sh(returnStdout: true, script: 'which python3.6').trim()

                buildAndTest(python27path, "venv27")
                buildAndTest(python34path, "venv34")
                buildAndTest(python35path, "venv35")
                buildAndTest(python36path, "venv36")
           }
        }

        parallel builders
    }

    if(branchName=="master") {
        stage('Upload assets') {
            builders['linux-x86_64'] = {
                node('jenkins-slave-ec2') {
                    def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                    def python34path = sh(returnStdout: true, script: 'which python3.4').trim()

                    uploadAsset(python27path, "venv27", "sdist")
                    uploadAsset(python27path, "venv27")
                    uploadAsset(python34path, "venv34")
                }
            }

            builders['apple-macos-compile'] = {
               node('macos') {
                    def python27path = sh(returnStdout: true, script: 'which python2.7').trim()
                    def python34path = sh(returnStdout: true, script: 'which python3.4').trim()
                    def python35path = sh(returnStdout: true, script: 'which python3.5').trim()
                    def python36path = sh(returnStdout: true, script: 'which python3.6').trim()

                    uploadAsset(python27path, "venv27")
                    uploadAsset(python34path, "venv34")
                    uploadAsset(python35path, "venv35")
                    uploadAsset(python36path, "venv36")
               }
            }

            parallel builders
        }
    }

    performReleaseIfNeeded()
}
