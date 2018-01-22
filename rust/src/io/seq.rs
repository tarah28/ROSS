/// A sequence struct that contains the ID, sequence, and quality cigar line
pub struct Seq {
  pub id:   String,
  pub seq:  String,
  pub qual: String,
}

/// A sequence that can be cleaned
pub trait Cleanable{
    /// new sequence object
    fn new (id: String, seq: String, qual: String) -> Seq;
    /// lower any low quality base to a zero and "N"
    fn lower_ambiguity_q(&mut self) -> ();
    /// Trim sequences based on quality
    fn trim(&mut self)  -> ();
    /// Reports bool whether the read passes thresholds.
    fn is_high_quality(&mut self) -> bool; 
    /// Make a String object
    fn to_string(&self) -> String;
    /// Print the result of to_string()
    fn print(&self)     -> ();
}


impl Cleanable for Seq {
    /// Make a new cleanable sequence object
    fn new (id: String, seq: String, qual: String) -> Seq{
        let mut id_copy = id.clone();
        if id_copy.chars().nth(0).expect("ID was empty") == '@' {
            id_copy.pop();
        }
        return Seq{
            id:   id_copy,
            seq:  seq.clone(),
            qual: qual.clone(),
        };
    }

    /// Alter any ambiguity site with a quality=0
    fn lower_ambiguity_q(&mut self){
        let zero_score:char  = 33 as char;
        let low_score :char  = (33 + 20) as u8 as char;

        let mut low_qual_idx = vec![false; self.seq.len()];
        // First find the indices b/c it is so slow to
        // edit a string in-place one char at a time.
        for (i,nt) in self.seq.chars().enumerate(){
            if nt == 'N' || nt == 'n' || self.qual.chars().nth(i).expect("Expected a char") < low_score {
                low_qual_idx[i] = true;
            }
        }
        
        let mut new_seq =String::new();
        let mut new_qual=String::new();
        for (i,nt) in self.seq.chars().enumerate(){
            if low_qual_idx[i] {
                new_seq.push('N');
                new_qual.push(zero_score);
            } else{
                new_seq.push(nt);
                new_qual.push_str(&self.qual[i..i+1]);
            }
        }

        self.seq=new_seq;
        self.qual=new_qual;
    }

    /// Trim the ends of reads with low quality
    fn trim(&mut self) {
        let min_qual = 20;

        let mut trim5=0;
        let mut trim3=&self.qual.len()-0;
        
        // 5'
        for qual in self.qual.chars(){
            if qual as u8 - 33 < min_qual {
                trim5+=1;
            } else {
                break;
            }
        }

        // 3'
        for qual in self.qual.chars().rev() {
            if qual as u8 - 33 < min_qual {
                trim3-=1;
            } else {
                break;
            }
        }
        
        self.qual = self.qual[trim5..trim3].to_string();
        self.seq  = self.seq[trim5..trim3].to_string();
    }

    /// Reports bool whether the read passes thresholds.
    fn is_high_quality(&mut self) -> bool {
        return true;
    }

    fn to_string(&self) -> String {
        let mut entry = String::new();
        if self.id.chars().nth(0).expect("Seq ID was not set") != '@' {
            entry.push('@');
        }
        entry.push_str(self.id.trim());
        entry.push_str("\n");
        entry.push_str(self.seq.trim());
        entry.push_str("\n+\n");
        entry.push_str(&self.qual.trim());
        return entry;
    }
    fn print(&self) -> () {
        println!("{}",self.to_string());
    }
} 

