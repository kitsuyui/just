use super::*;

#[test]
#[ignore]
fn prior_dependencies_run_in_parallel() {
  let start = Instant::now();

  Test::new()
    .justfile(
      "
        [parallel]
        foo: a b c d e

        a:
          sleep 1

        b:
          sleep 1

        c:
          sleep 1

        d:
          sleep 1

        e:
          sleep 1
      ",
    )
    .stderr(
      "
        sleep 1
        sleep 1
        sleep 1
        sleep 1
        sleep 1
      ",
    )
    .success();

  assert!(start.elapsed() < Duration::from_secs(2));
}

#[test]
#[ignore]
fn subsequent_dependencies_run_in_parallel() {
  let start = Instant::now();

  Test::new()
    .justfile(
      "
        [parallel]
        foo: && a b c d e

        a:
          sleep 1

        b:
          sleep 1

        c:
          sleep 1

        d:
          sleep 1

        e:
          sleep 1
      ",
    )
    .stderr(
      "
        sleep 1
        sleep 1
        sleep 1
        sleep 1
        sleep 1
      ",
    )
    .success();

  assert!(start.elapsed() < Duration::from_secs(2));
}

#[test]
fn parallel_dependencies_report_errors() {
  Test::new()
    .justfile(
      "
        [parallel]
        foo: bar

        bar:
          exit 1
      ",
    )
    .stderr(
      "
        exit 1
        error: Recipe `bar` failed on line 5 with exit code 1
      ",
    )
    .failure();
}

#[test]
#[cfg(not(windows))]
fn parallel_dependency_failure_terminates_siblings() {
  let output = Test::new()
    .justfile(
      "
        [parallel]
        foo: fail side_effect

        fail:
          false

        side_effect:
          sleep 1
          echo side-effect > out
      ",
    )
    .stderr_regex(
      "(false\nsleep 1|sleep 1\nfalse)\nerror: Recipe `fail` failed on line 5 with exit code 1\n",
    )
    .failure();

  assert!(!output.tempdir.path().join("out").exists());
}

#[test]
#[ignore]
fn dependents_block_on_running_dependencies() {
  Test::new()
    .justfile(
      "
        set quiet

        [parallel]
        a: b c
          echo a

        b: x
          echo b

        c: x
          echo c

        x:
          sleep 1
          echo x
      ",
    )
    .stdout_regex(
      r"(?x)
      x\n
      (
        b\nc\n
        |
        c\nb\n
      )
      a\n",
    )
    .success();
}
