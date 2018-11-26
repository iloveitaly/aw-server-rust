extern crate chrono;
extern crate serde_json;

extern crate aw_server;

#[cfg(test)]
mod transform_tests {
    use std::str::FromStr;

    use chrono::Utc;
    use chrono::DateTime;
    use chrono::Duration;
    use serde_json::json;

    use aw_server::models::Event;
    use aw_server::transform;

    #[test]
    fn test_heartbeat_pulsetime() {
        let now = Utc::now();
        let event1 = Event {
            id: None,
            timestamp: now,
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let heartbeat1 = Event {
            id: None,
            timestamp: now + Duration::seconds(2),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };

        // Merge result
        let res_merge = transform::heartbeat(&event1, &heartbeat1, 2.0).unwrap();
        assert!(res_merge.duration == Duration::seconds(3));

        // No merge result
        let res_no_merge = transform::heartbeat(&event1, &heartbeat1, 0.0);
        assert!(res_no_merge.is_none());

        // TODO: needs more tests!
    }

    #[test]
    fn test_heartbeat_data() {
        let now = Utc::now();
        let event = Event {
            id: None,
            timestamp: now.clone(),
            duration: Duration::seconds(0),
            data: json!({"test": 1})
        };
        let heartbeat_same_data = Event {
            id: None,
            timestamp: now.clone(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };

        // Data is same, should merge
        let res_merge = transform::heartbeat(&event, &heartbeat_same_data, 1.0);
        assert!(res_merge.is_some());

        let heartbeat_different_data = Event {
            id: None,
            timestamp: now.clone(),
            duration: Duration::seconds(1),
            data: json!({"test": 2})
        };
        // Data is different, should not merge
        let res_merge = transform::heartbeat(&event, &heartbeat_different_data, 1.0);
        assert!(res_merge.is_none());
    }

    #[test]
    fn test_sort_by_timestamp() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let e2 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:03Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let res = transform::sort_by_timestamp(vec![e2.clone(), e1.clone()]);
        assert_eq!(res, vec![e1, e2]);
    }

    #[test]
    fn test_sort_by_duration() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(2),
            data: json!({"test": 1})
        };
        let e2 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:03Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let res = transform::sort_by_duration(vec![e2.clone(), e1.clone()]);
        assert_eq!(res, vec![e1, e2]);
    }

    #[test]
    fn test_flood() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let e2 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:03Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let e_expected = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(4),
            data: json!({"test": 1})
        };
        let res = transform::flood(vec![e1.clone(), e2.clone()], Duration::seconds(5));
        // TODO: check result
        let res_e = &res[0];
        assert_eq!(res_e, &e_expected);
    }

    #[test]
    fn test_merge_events_by_key() {
        let e1 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
            duration: Duration::seconds(1),
            data: json!({"test": 1})
        };
        let e2 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:01Z").unwrap(),
            duration: Duration::seconds(3),
            data: json!({"test2": 3})
        };
        let e3 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:02Z").unwrap(),
            duration: Duration::seconds(7),
            data: json!({"test": 6})
        };
        let e4 = Event {
            id: None,
            timestamp: DateTime::from_str("2000-01-01T00:00:03Z").unwrap(),
            duration: Duration::seconds(9),
            data: json!({"test": 1})
        };
        let in_events = vec![e1.clone(), e2.clone(), e3.clone(), e4.clone()];
        let res1 = transform::merge_events_by_keys (in_events, vec!["test".to_string()]);
        // Needed, otherwise the order is undeterministic
        let res2 = transform::sort_by_timestamp (res1);
        let expected = vec![
            Event {
                id: None,
                timestamp: DateTime::from_str("2000-01-01T00:00:00Z").unwrap(),
                duration: Duration::seconds(10),
                data: json!({"test": 1})
            },
            Event {
                id: None,
                timestamp: DateTime::from_str("2000-01-01T00:00:02Z").unwrap(),
                duration: Duration::seconds(7),
                data: json!({"test": 6})
            }
        ];
        assert_eq!(&res2, &expected);
    }
}
